set dotenv-load

build-example EXAMPLE FEATURES="" TARGET="msp430-none-elf" OVERRIDE="nightly" WORKSPACE="take-api":
    #!/bin/sh

    set -eux

    if [ {{TARGET}} = "msp430-none-elf" ]; then
        OBJDUMP="msp430-elf-objdump"
        READELF="msp430-elf-readelf"
        SIZE="msp430-elf-size"
    elif [ {{TARGET}} = "thumbv6m-none-eabi" ]; then
        OBJDUMP="llvm-objdump"
        READELF="llvm-readelf"
        SIZE="llvm-size"
    else
        echo "Unsupport target {{TARGET}}"
        exit 1
    fi

    if [ -z {{EXAMPLE}} ]; then
        EXAMPLES=""
        TARGET=target/{{TARGET}}/release/{{WORKSPACE}}
    else
        EXAMPLES="--example={{EXAMPLE}}"
        TARGET=target/{{TARGET}}/release/examples/{{EXAMPLE}}
    fi

    cargo +{{OVERRIDE}} rustc --manifest-path=./{{WORKSPACE}}/Cargo.toml --target={{TARGET}} --release -Zbuild-std=core $EXAMPLES --features={{FEATURES}} -- --emit=obj=$TARGET.o,llvm-ir=$TARGET.ll,asm=$TARGET.s
    $OBJDUMP -Cd $TARGET > $TARGET.lst
    $READELF -a --wide $TARGET > $TARGET.sym
    $OBJDUMP -Cd $TARGET.o > $TARGET.o.lst
    $READELF -a --wide $TARGET.o > $TARGET.reloc
    $SIZE $TARGET

# Compare sizes using rustup
compare-min OVERRIDE="nightly":
    cargo +{{OVERRIDE}} rustc --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core --example min -- --emit=llvm-ir=compare/min-nobare.ll,asm=compare/min-nobare.asm
    msp430-elf-size -A target/msp430-none-elf/release/examples/min | tee compare/min-nobare.size
    cargo +{{OVERRIDE}} rustc --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core --example min --features use-bare-metal -- --emit=llvm-ir=compare/min-bare.ll,asm=compare/min-bare.asm
    msp430-elf-size -A target/msp430-none-elf/release/examples/min | tee compare/min-bare.size

# Compare sizes using manually-specified cargo and Rust (in env var).
compare-min-cargo:
    RUSTC_BOOTSTRAP=1 RUSTC=$RUSTC_OVERRIDE $CARGO rustc --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core --example min -- --emit=llvm-ir=compare/min-nobare.ll,asm=compare/min-nobare.asm
    msp430-elf-size -A target/msp430-none-elf/release/examples/min | tee compare/min-nobare.size
    RUSTC_BOOTSTRAP=1 RUSTC=$RUSTC_OVERRIDE $CARGO rustc --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf -Z build-std=core --example min --features use-bare-metal -- --emit=llvm-ir=compare/min-bare.ll,asm=compare/min-bare.asm
    msp430-elf-size -A target/msp430-none-elf/release/examples/min | tee compare/min-bare.size

# Compare sizes using xargo, plus manually-specified cargo and Rust (in env var).
compare-min-xargo:
    RUSTC_BOOTSTRAP=1 RUSTC=$RUSTC_OVERRIDE PATH=$XARGO_CARGO_PATH:$PATH xargo rustc --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf --example min -- --emit=llvm-ir=compare/min-nobare.ll,asm=compare/min-nobare.asm
    msp430-elf-size -A target/msp430-none-elf/release/examples/min | tee compare/min-nobare.size
    RUSTC_BOOTSTRAP=1 RUSTC=$RUSTC_OVERRIDE PATH=$XARGO_CARGO_PATH:$PATH xargo rustc --manifest-path=take-api/Cargo.toml --release --target=msp430-none-elf --example min --features use-bare-metal -- --emit=llvm-ir=compare/min-bare.ll,asm=compare/min-bare.asm
    msp430-elf-size -A target/msp430-none-elf/release/examples/min | tee compare/min-bare.size

clean OVERRIDE="nightly":
    cargo +{{OVERRIDE}} clean

bisect START END SCRIPT:
    cargo-bisect-rustc --script={{SCRIPT}} --preserve-target --preserve --start={{START}} --end={{END}} --with-src

uninstall-bisect:
    rustup toolchain list | grep bisector | xargs -n 1 rustup toolchain uninstall

clean-bisect:
    rm -rf target-bisector-nightly-*
    rm -rf target-bisector-ci-*

zip SUFFIX="":
    #!/bin/sh

    set -eu

    if [ -z {{SUFFIX}} ]; then
        SUFFIX=`rustc -V | sed 's/.*(\([0-9a-f]*\).*)/\1/'`
    else
        SUFFIX=""
    fi

    if [ -e target-$SUFFIX.zip ]; then
        echo "target-$SUFFIX.zip already exists"
        exit 1
    fi

    zip -r target-$SUFFIX.zip target
    echo "target-$SUFFIX.zip"
