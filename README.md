# Mac screen/keyboard brightness controller

## How to compile

```bash
cargo clean && \
cargo build --release && \
strip ./target/release/mac-light-controller
```

## How to run

- It will print the usage if no parameter provided

    ```bash
    ./target/release/mac-light-controller
    Usage: [DEBUG=true] mac-light-controller keyboard/screen +/- [total_steps_from_0_to_max]

    - 'keyboard/screen' is required.
    - '+/-' is required.
    - If 'DEBUG=true' provide, it prints the debug information.
    - If `total_steps_from_0_to_max` not provided, set to `10` by default which means press 10 times from `0` brightness to `max` brightness.
    ```

- Increase screen brightness
    ```bash
    DEBUG=true ./target/release/mac-light-controller Screen +
    # brightness_control_type: Screen
    # is_increase_brightness: true
    # total_steps: 10
    # max_brightness: 1953
    # current_brightness: 1195
    # temp_step_value: 195
    # new_brightness_value: 1390
    # Set new brightness value '1390' successfully.
    ```

- Decrease screen brightness
    ```bash
    DEBUG=true ./target/release/mac-light-controller Screen -
    # brightness_control_type: Screen
    # is_increase_brightness: false
    # total_steps: 10
    # max_brightness: 1953
    # current_brightness: 1390
    # temp_step_value: 195
    # new_brightness_value: 1195
    # Set new brightness value '1195' successfully.
    ```

- Increase keyboard brightness
    ```bash
    DEBUG=true ./target/release/mac-light-controller Keyboard +
    # brightness_control_type: Keyboard
    # is_increase_brightness: true
    # total_steps: 10
    # max_brightness: 255
    # current_brightness: 0
    # temp_step_value: 25
    # new_brightness_value: 25
    # Set new brightness value '25' successfully.
    ```

- Decrease keyboard brightness
    ```bash
    DEBUG=true ./target/release/mac-light-controller Keyboard -
    # brightness_control_type: Keyboard
    # is_increase_brightness: false
    # total_steps: 10
    # max_brightness: 255
    # current_brightness: 25
    # temp_step_value: 25
    # new_brightness_value: 0
    # Set new brightness value '0' successfully.
    ```
