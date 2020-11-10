#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use rias_os::serial_println;
use rias_os::arch;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(target_arch = "x86_64"))]
    serial_println!("{} skipped, wrong arch!", module_path!());
    #[cfg(not(target_arch = "x86_64"))]
        arch::shutdown_qemu(true);

    serial_println!("Running {} integration tests", module_path!());

    arch::startup();

    // trigger a stack overflow
    stack_overflow();

    arch::shutdown_qemu(false);
    loop {}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!(".[ok]");
    arch::shutdown_qemu(true);
    loop {}
}