mod rias_x86_64{
    pub mod boot;
    pub mod shutdown;
}

pub fn startup() {
    #[cfg(target_arch = "x86_64")]
    use rias_x86_64::boot;
    
    boot::boot();
    boot::enable();
}

pub fn halt() -> ! {
    #[cfg(target_arch = "x86_64")]
    use rias_x86_64::shutdown;

    shutdown::halt();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn shutdown_qemu(success: bool) {
    #[cfg(target_arch = "x86_64")]
    use rias_x86_64::shutdown;

    shutdown::exit_qemu(if success == true { QemuExitCode::Success } else { QemuExitCode::Failed });
}