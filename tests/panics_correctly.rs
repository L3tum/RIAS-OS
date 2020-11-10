#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rias_os::{arch, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!(".[ok]");
    arch::shutdown_qemu(true);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Running {} integration tests", module_path!());
    serial_println!("Running {} tests", 1);

    should_fail();

    arch::shutdown_qemu(true);
    loop {}
}

fn should_fail() {
    assert_eq!(0, 1);
}