mod rias_x86_64 {
    pub mod boot;
    pub mod shutdown;
    pub mod instructions;
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

pub fn disable_interrupts() {
    #[cfg(target_arch = "x86_64")]
    use rias_x86_64::instructions;

    instructions::disable_interrupts();
}

pub fn enable_interrupts() {
    #[cfg(target_arch = "x86_64")]
    use rias_x86_64::instructions;

    instructions::enable_interrupts();
}

pub fn enable_interrupts_and_halt() {
    #[cfg(target_arch = "x86_64")]
    use rias_x86_64::instructions;

    instructions::enable_interrupts_and_halt();
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