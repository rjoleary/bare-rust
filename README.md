Minimal code for running Rust code on ARM without an OS.

Setup:

    # Install gcc cross-compiler
    sudo apt-get install gcc-arm-none-eabi

    # Install rustup
    curl https://sh.rustup.rs -sSf | sh

    # Install rust nightly and ARM toolchain
    rustup toolchain install nightly
    rustup override set nightly
    rustup target add arm-unknown-linux-gnueabihf

Building:

    rustc --target=arm-unknown-linux-gnueabihf -O --emit asm main.rs
    /usr/bin/arm-none-eabi-as main.s -o main

QEMU:

    qemu-system-arm --machine virt --kernel main

    # Quit qemu with CTRL-A X
