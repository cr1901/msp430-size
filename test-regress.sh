#!/bin/sh
echo "Not yet implemented"
exit 1

NOBM_FILE=`mktemp`
BM_FILE=`mktemp`

cargo build --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core $EXAMPLE $FEATURES_GOOD
msp430-elf-size -A $CARGO_TARGET_DIR/msp430-none-elf/release/$EXAMPLE > $BM_FILE

cargo build --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core $EXAMPLE $FEATURES_BAD
msp430-elf-size -A $CARGO_TARGET_DIR/msp430-none-elf/release/$EXAMPLE > $NOBM_FILE

diff -q $NOBM_FILE $BM_FILE
RV=$?

rm $NOBM_FILE $BM_FILE

exit $RV
