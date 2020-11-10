#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(rias_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rias_os::{serial_println, arch};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(target_arch = "x86_64"))]
        serial_println!("{} skipped, wrong arch!", module_path!());
    #[cfg(not(target_arch = "x86_64"))]
        arch::shutdown_qemu(true);

    serial_println!("Running {} integration tests", module_path!());
    arch::startup();
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rias_os::test_panic_handler(info)
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}