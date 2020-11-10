#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(rias_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;

use core::panic::PanicInfo;
use rias_os::{
    arch,
    println,
    debug_println,
    self,
    memory,
};
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    arch::halt();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rias_os::test_panic_handler(info);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    use rias_os::serial_println;
    #[cfg(test)]
    serial_println!("Running {} integration tests", module_path!());
    #[cfg(test)]
        test_main();

    debug_println!("Starting hardware initialization...");
    arch::startup();

    debug_println!("Setting up memory mapping...");
    debug_println!("Physical memory offset {:X}", boot_info.physical_memory_offset);

    unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset), &boot_info.memory_map) }

    println!("Hello World!");

    Box::new(1);

    end();
}

fn end() -> ! {
    arch::halt();
}