set dotenv-load

build-example EXAMPLE OVERRIDE="nightly" WORKSPACE="take-api":
    #!/bin/sh

    set -eux

    TARGET=target/msp430-none-elf/release/examples/{{EXAMPLE}}
    cargo +{{OVERRIDE}} rustc --manifest-path=./{{WORKSPACE}}/Cargo.toml --release -Zbuild-std=core --example={{EXAMPLE}} -- --emit=obj=$TARGET.o,llvm-ir=$TARGET.ll,asm=$TARGET.s
    msp430-elf-objdump -Cd $TARGET > $TARGET.lst
    msp430-elf-readelf -a --wide $TARGET > $TARGET.sym
    msp430-elf-objdump -Cd $TARGET.o > $TARGET.o.lst
    msp430-elf-readelf -a --wide $TARGET.o > $TARGET.reloc
    msp430-elf-size $TARGET

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
