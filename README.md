# Mac screen brightness controller

## How to compile

```bash
cargo clean && \
cargo build --release && \
strip ./target/release/mac-screen-brightness-controller
```

## How to run

- It will print the usage if no parameter provided

    ```bash
    ./target/release/mac-screen-brightness-controller
    Usage: [DEBUG=true] mac-screen-brightness-controller [total_steps_from_0_to_max]

    - '+/-' is required.
    - If 'DEBUG=true' provide, it prints the debug information.
    - If `total_steps_from_0_to_max` not provided, set to `10` by default which means press 10 times from `0` brightness to `max` brightness.
    ```

- Increase screen brightness
    ```bash
    DEBUG=true ./target/release/mac-screen-brightness-controller +

    # is_add_brightness: true
    # total_steps: 10
    # max_brightness: Some(1953)
    # current_brightness: Some(390)
    # temp_step_value: 195
    # new_brightness_value: 585
    # Set new brightness value '585' successfully.
    ```

- Decrease screen brightness
    ```bash
    DEBUG=true ./target/release/mac-screen-brightness-controller -

    # is_add_brightness: false
    # total_steps: 10
    # max_brightness: Some(1953)
    # current_brightness: Some(585)
    # temp_step_value: 195
    # new_brightness_value: 390
    # Set new brightness value '390' successfully.
    ```
