#![allow(warnings)]
use std::env;
use std::fs;
use std::process::Command;

type CommandResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INTEL_BRIGHTNESS_CURRENT_VALUE_SYSFS_PATH: &'static str =
    "/sys/class/backlight/intel_backlight/brightness";
// "/home/wison/temp/test.log";

const INTEL_BRIGHTNESS_MAX_VALUE_SYSFS_PATH: &'static str =
    "/sys/class/backlight/intel_backlight/max_brightness";

const USAGE_TIPS: &'static str = "Usage: [DEBUG=true] mac-screen-brightness-controller [total_steps_from_0_to_max]\n\n\
- '+/-' is required.\n\
- If 'DEBUG=true' provide, it prints the debug information.\n\
- If `total_steps_from_0_to_max` not provided, set to `10` by default which means press 10 times from `0` brightness to `max` brightness.
";

/// Usage: [DEBUG=true] mac-screen-brightness-controller +/- [total_steps_from_0_to_max]
///
/// - '+/-' is required
/// - If 'DEBUG=true' provide, it prints the debug information.
/// - If `total_steps_from_0_to_max` not provided, set to `10` by default which means press 10 times
/// from `0` brightness to `max` brightness.
fn main() -> CommandResult<()> {
    let mut args = env::args();
    if args.len() <= 1 {
        println!("{}", USAGE_TIPS);
        return Ok(());
    }

    let enable_debug = match env::var("DEBUG") {
        Ok(value) => value.trim().to_lowercase() == "true",
        Err(_) => false,
    };

    let mut is_increase_brightness = true;
    let mut total_steps: u32 = 10;
    for (index, arg_value) in args.enumerate() {
        if index == 1 {
            is_increase_brightness = arg_value.trim() == "+";
        } else if index == 2 {
            total_steps = arg_value
                .parse::<u32>()
                .expect("'total_steps_from_0_to_max' value must be an integer.")
        }
    }

    if enable_debug {
        println!("is_increase_brightness: {}", is_increase_brightness);
        println!("total_steps: {}", total_steps);
    }

    // 1. Read the max value
    let shell_command_for_reading_max_brightness =
        format!("cat {}", INTEL_BRIGHTNESS_MAX_VALUE_SYSFS_PATH);
    let read_max_brightness_result = Command::new("sh")
        .arg("-c")
        .arg(shell_command_for_reading_max_brightness)
        .output()?;

    let mut max_brightness: Option<u32> = None;

    if read_max_brightness_result.status.success() {
        let mut command_stdout = read_max_brightness_result.stdout;
        if command_stdout.len() > 1 {
            command_stdout.truncate(command_stdout.len() - 1);
        }

        max_brightness = Some(String::from_utf8(command_stdout)?.parse::<u32>().unwrap());
        if enable_debug {
            println!("max_brightness: {:?}", max_brightness);
        }
    } else {
        let error_message = String::from_utf8(read_max_brightness_result.stderr)?;
        println!("read max brightness error: {}", error_message);
    }

    // 2. Read the current value
    let shell_command_for_reading_current_brightness =
        format!("cat {}", INTEL_BRIGHTNESS_CURRENT_VALUE_SYSFS_PATH);
    let read_current_brightness_result = Command::new("sh")
        .arg("-c")
        .arg(shell_command_for_reading_current_brightness)
        .output()?;

    let mut current_brightness: Option<u32> = None;

    if read_current_brightness_result.status.success() {
        let mut command_stdout = read_current_brightness_result.stdout;
        if command_stdout.len() > 1 {
            command_stdout.truncate(command_stdout.len() - 1);
        }

        current_brightness = Some(String::from_utf8(command_stdout)?.parse::<u32>().unwrap());
        if enable_debug {
            println!("current_brightness: {:?}", current_brightness);
        }
    } else {
        let error_message = String::from_utf8(read_current_brightness_result.stderr)?;
        println!("read current brightness error: {}", error_message);
    }

    // 3. Calculate the next value and write it back
    let temp_step_value = (max_brightness.as_ref().unwrap() / total_steps) as u32;
    let current_brightness_int = current_brightness.unwrap();
    let max_brightness_int = max_brightness.unwrap();

    let mut new_brightness_value = if is_increase_brightness {
        if current_brightness_int + temp_step_value > max_brightness_int {
            max_brightness_int
        } else {
            current_brightness_int + temp_step_value
        }
    } else {
        if (current_brightness_int as i32) - (temp_step_value as i32) < 0 {
            0
        } else {
            current_brightness_int - temp_step_value
        }
    };

    if enable_debug {
        println!("temp_step_value: {}", temp_step_value);
        println!("new_brightness_value: {}", new_brightness_value);
    }

    let write_result = fs::write(
        INTEL_BRIGHTNESS_CURRENT_VALUE_SYSFS_PATH,
        new_brightness_value.to_string(),
    )?;
    //
    if enable_debug {
        println!(
            "Set new brightness value '{}' successfully.",
            new_brightness_value
        );
    }

    Ok(())
}
