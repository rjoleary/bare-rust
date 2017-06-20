#![feature(core_intrinsics)]
#![feature(libc)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
use core::intrinsics;

extern crate libc;

fn putc(data: u8) {
    unsafe {
        // Uart address is specific to qemu's virt machine.
        core::intrinsics::volatile_store(0x09000000 as *mut u32, data as u32);
    }
}

#[start]
#[no_mangle]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    for &i in b"0123456789" {
        putc(i);
    }
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_msg: core::fmt::Arguments,
                        _file: &'static str,
                        _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}
