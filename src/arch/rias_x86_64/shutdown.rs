pub fn exit_qemu(exit_code: crate::arch::QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}