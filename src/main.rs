#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(rias_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

use rias_os::{
    self,
    arch,
    debug_println,
    memory,
    println,
};
use rias_os::task::{keyboard, Task};
use rias_os::task::executor::Executor;
use rias_os::task::simple_executor::SimpleExecutor;

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

    debug_println!("Starting executor");
    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    end();
}

fn end() -> ! {
    arch::halt();
}