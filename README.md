# rust-debug-2023 

This project showcases an embedded debugging workflow on Rust. It is used as a learning project in debugging rust on the thingy91 using a blackmagic debug probe.

For a detailed explanation, see the corresponding article here https://betterprogramming.pub/debugging-embedded-rust-e92ff0b8b8e5#2139-4c07ac7e0c93.

## Hardware set-up 

I am using a Blackmagic debug probe in combination with a thingy91 as my hardware set-up. You can use a nrf9160 development kit as well or connect a JLink or other probe to the thingy91. For the BMP I am using the `.vscode/launch.json` configuration to auto-load the firmware and launc the application.

I'm using the built-in RTT device from the BMP to check my RTT output, using `screen /dev/tty.usbmodem98B724953`.

