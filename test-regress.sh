#!/bin/sh
NOBM_FILE=`mktemp`
BM_FILE=`mktemp`

cargo build --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core --example min --features use-bare-metal
msp430-elf-size -A $CARGO_TARGET_DIR/msp430-none-elf/release/examples/min > $BM_FILE

cargo build --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core --example min
msp430-elf-size -A $CARGO_TARGET_DIR/msp430-none-elf/release/examples/min > $NOBM_FILE

diff -q $NOBM_FILE $BM_FILE
RV=$?

rm $NOBM_FILE $BM_FILE

exit $RV
