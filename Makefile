.PHONY: build run clean

build: main

main: main.s
	/usr/bin/arm-none-eabi-as main.s -o main

main.s: main.rs
	rustc --target=arm-unknown-linux-gnueabihf -O --emit asm main.rs

run: main
	qemu-system-arm --machine virt --kernel main --nographic

clean:
	rm -f main.s main
