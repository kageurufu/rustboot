#![feature(core)]
#![feature(core_prelude)]
//#![feature(no_std)]
#![feature(intrinsics)]
#![feature(asm)]
#![feature(link_args)]

extern crate core;

use uefi::*;

static RUSTBOOT_NAME    : &'static str = "RustBoot v0.0.1";

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_copy_implementations)]
pub mod uefi;

#[no_stack_check]
pub fn efi_main(sys: uefi::SystemTable) {
    let cons = sys.console();

    cons.write_many(&[
                    RUSTBOOT_NAME,
                    "\r\n"
    ]);
    cons.debug("MAIN", "Locating boot entries");
    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}

#[no_mangle]
pub fn breakpoint() -> ! {
    loop {}
}

