#!/bin/sh
echo "Not yet implemented"
exit 1
cargo build --manifest-path=test-cases/Cargo.toml --release --target=msp430-none-elf -Z build-std=core --example min
