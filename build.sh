#!/usr/bin/bash


# x86_64 Linux
cargo build --release --target x86_64-unknown-linux-gnu
mv target/release/telegram-send-msg bin/telegram-send-msg-linux
# ARM Linux (aarch64)
cargo build --release --target aarch64-unknown-linux-gnu
mv target/release/telegram-send-msg bin/telegram-send-msg-rpi
# x86_64 Windows
cargo build --release --target x86_64-pc-windows-gnu
mv target/release/telegram-send-msg bin/telegram-send-msg.exe
