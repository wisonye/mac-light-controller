// #![allow(warnings)]
use std::env;
use std::fs;

type CommandResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Screen brightness
const INTEL_BRIGHTNESS_CURRENT_VALUE_SYSFS_PATH: &'static str =
    "/sys/class/backlight/intel_backlight/brightness";
const INTEL_BRIGHTNESS_MAX_VALUE_SYSFS_PATH: &'static str =
    "/sys/class/backlight/intel_backlight/max_brightness";

// Keyboard LED light
const KEYBOARD_LIGHT_MAX_VALUE_SYSFS_PATH: &'static str =
    "/sys/class/leds/smc::kbd_backlight/max_brightness";
const KEYBOARD_LIGHT_CURRENT_VALUE_SYSFS_PATH: &'static str =
    "/sys/class/leds/smc::kbd_backlight/brightness";

const USAGE_TIPS: &'static str = "Usage: [DEBUG=true] mac-light-controller keyboard/screen +/- [total_steps_from_0_to_max]\n\n\
- 'keyboard/screen' is required.\n\
- '+/-' is required.\n\
- If 'DEBUG=true' provide, it prints the debug information.\n\
- If `total_steps_from_0_to_max` not provided, set to `10` by default which means press 10 times from `0` brightness to `max` brightness.
";

/// Read a value from sys file and. It will return an `Vec<u8>`, truncate to keep the first line.
fn read_int_value_from_sys_file(sys_fs_path: &str) -> CommandResult<u32> {
    let mut temp_vec = fs::read(sys_fs_path)?;
    if temp_vec.len() > 1 {
        temp_vec.truncate(temp_vec.len() - 1);
    }

    Ok(String::from_utf8(temp_vec)?.parse::<u32>()?)
}

///
#[derive(Debug)]
enum BrightnessControlType {
    Keyboard,
    Screen,
    Unsupported,
}

/// Usage: [DEBUG=true] mac-light-controller keyboard/screen +/- [total_steps_from_0_to_max]
///
/// - 'keyboard/screen' is required.
/// - '+/-' is required.
/// - If 'DEBUG=true' provide, it prints the debug information.
/// - If `total_steps_from_0_to_max` not provided, set to `10` by default which means press 10 times
/// from `0` brightness to `max` brightness.
fn main() -> CommandResult<()> {
    let args = env::args();
    if args.len() <= 2 {
        println!("{}", USAGE_TIPS);
        return Ok(());
    }

    let enable_debug = match env::var("DEBUG") {
        Ok(value) => value.trim().to_lowercase() == "true",
        Err(_) => false,
    };

    let mut control_type = BrightnessControlType::Unsupported;
    let mut is_increase_brightness = true;
    let mut total_steps: u32 = 10;
    for (index, arg_value) in args.enumerate() {
        if index == 1 {
            let temp_str = arg_value.trim().to_lowercase();
            match temp_str.as_str() {
                "keyboard" => control_type = BrightnessControlType::Keyboard,
                "screen" => control_type = BrightnessControlType::Screen,
                _ => panic!(format!("Unsupported type: {}", temp_str)),
            }
        } else if index == 2 {
            is_increase_brightness = arg_value.trim() == "+";
        } else if index == 2 {
            total_steps = arg_value
                .parse::<u32>()
                .expect("'total_steps_from_0_to_max' value must be an integer.")
        }
    }

    if enable_debug {
        println!("brightness_control_type: {:?}", &control_type);
        println!("is_increase_brightness: {}", is_increase_brightness);
        println!("total_steps: {}", total_steps);
    }

    // 1. Read the max value
    let max_brightness_sys_fs_path = match control_type {
        BrightnessControlType::Screen => INTEL_BRIGHTNESS_MAX_VALUE_SYSFS_PATH,
        BrightnessControlType::Keyboard => KEYBOARD_LIGHT_MAX_VALUE_SYSFS_PATH,
        _ => panic!("Unsupported control type"),
    };

    let max_brightness: u32 = read_int_value_from_sys_file(max_brightness_sys_fs_path)?;

    if enable_debug {
        println!("max_brightness: {}", max_brightness);
    }

    // 2. Read the current value
    let current_brightness_sys_fs_path = match control_type {
        BrightnessControlType::Screen => INTEL_BRIGHTNESS_CURRENT_VALUE_SYSFS_PATH,
        BrightnessControlType::Keyboard => KEYBOARD_LIGHT_CURRENT_VALUE_SYSFS_PATH,
        _ => panic!("Unsupported control type"),
    };

    let current_brightness: u32 = read_int_value_from_sys_file(current_brightness_sys_fs_path)?;

    if enable_debug {
        println!("current_brightness: {}", current_brightness);
    }

    // 3. Calculate the next value and write it back
    let temp_step_value = (max_brightness / total_steps) as u32;

    let new_brightness_value = if is_increase_brightness {
        if current_brightness + temp_step_value > max_brightness {
            max_brightness
        } else {
            current_brightness + temp_step_value
        }
    } else {
        if (current_brightness as i32) - (temp_step_value as i32) < 0 {
            0
        } else {
            current_brightness - temp_step_value
        }
    };

    if enable_debug {
        println!("temp_step_value: {}", temp_step_value);
        println!("new_brightness_value: {}", new_brightness_value);
    }

    let _ = fs::write(
        match control_type {
            BrightnessControlType::Screen => INTEL_BRIGHTNESS_CURRENT_VALUE_SYSFS_PATH,
            BrightnessControlType::Keyboard => KEYBOARD_LIGHT_CURRENT_VALUE_SYSFS_PATH,
            _ => panic!("Unsupported control type"),
        },
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
