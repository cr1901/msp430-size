#!/bin/sh
NOBM_FILE=`mktemp`
BM_FILE=`mktemp`

xargo build --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf --example min --features use-bare-metal
msp430-elf-size -A $CARGO_TARGET_DIR/msp430-none-elf/release/examples/min > $BM_FILE

xargo build --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf --example min
msp430-elf-size -A $CARGO_TARGET_DIR/msp430-none-elf/release/examples/min > $NOBM_FILE

diff -q $NOBM_FILE $BM_FILE
RV=$?

rm $NOBM_FILE $BM_FILE

exit $RV
