#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rias_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use rias_os::{arch, memory};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    serial_println!("Running {} integration tests", module_path!());
    arch::startup();

    unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset), &boot_info.memory_map) }

    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rias_os::test_panic_handler(info)
}

use rias_os::{serial_print, serial_println};
use alloc::boxed::Box;

#[test_case]
fn simple_allocation() {
    let heap_value = Box::new(41);
    assert_eq!(*heap_value, 41);
}

use alloc::vec::Vec;
use rias_os::allocator::HEAP_SIZE;

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..10_000 {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1); // new
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1); // new
}