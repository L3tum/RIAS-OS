#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(llvm_asm)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(const_in_array_repeat_expressions)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate alloc;

pub mod drivers {
    pub mod vga;
    pub mod serial;
}

pub mod arch;
pub mod memory;
pub mod allocator;
pub mod config;

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_print!(".");
    }

    serial_println!("[ok]");
    arch::shutdown_qemu(true);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    arch::shutdown_qemu(false);
    loop {}
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    serial_println!("Running unit tests!");
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    // TODO: Swap memory?
    panic!("allocation error: {:?}", layout)
}