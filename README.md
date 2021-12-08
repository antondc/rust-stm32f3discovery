# Rust Stm32F3Discovery starting kit

Starting kit for STM32F3Discovery board: <https://www.st.com/resource/en/user_manual/dm00063382-discovery-kit-with-stm32f303vc-mcu-stmicroelectronics.pdf>

## Start OpenCD server

- Go to `/tmp` and use `openocd` to connect the device:

      cd /tmp
      openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

- On new tab start debug session with `cargo run` for this target:

      cargo run --target thumbv7em-none-eabihf -- -q -ex 'target remote :3333'
