use crate::{
    println,
    debug_println
};

pub mod idt;
pub mod gdt;
pub mod pic;

pub fn boot() {
    gather_hardware_info();
    debug_println!("Setting up memory regions...");
    gdt::init();
    debug_println!("Setting up interrupts...");
    idt::setup_idt();
    pic::setup_pic();
}

pub fn enable() {
    debug_println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
}

fn gather_hardware_info() {
    let cpuid = x86::cpuid::CpuId::new();

    let vendor = cpuid.get_vendor_info().unwrap();

    if vendor.as_string() == "AuthenticAMD" {
        match get_cpu_name_amd() {
            Some(x) => println!("CPU {} by {}", x.as_string(), vendor.as_string()),
            _ => println!("Could not determine CPU!")
        }
    }
}

fn get_cpu_name_amd() -> Option<CpuName> {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut eax: u32 = 0x80000002;
    let ecx: u32 = 0;

    unsafe {
        asm!("cpuid" : "={eax}"(a),"={ebx}"(b),"={ecx}"(c),"={edx}"(d) : "{eax}"(eax), "{ecx}"(ecx) : "eax","ebx","ecx","edx")
    }

    let mut cpu_name = CpuName { eax: a, ebx: b, ecx: c, edx: d, eax2: 0, ebx2: 0, ecx2: 0, edx2: 0, eax3: 0, ebx3: 0, ecx3: 0, edx3: 0 };

    eax = 0x80000003;

    unsafe {
        asm!("cpuid" : "={eax}"(a),"={ebx}"(b),"={ecx}"(c),"={edx}"(d) : "{eax}"(eax), "{ecx}"(ecx) : "eax","ebx","ecx","edx")
    }

    cpu_name.eax2 = a;
    cpu_name.ebx2 = b;
    cpu_name.ecx2 = c;
    cpu_name.edx2 = d;

    eax = 0x80000004;

    unsafe {
        asm!("cpuid" : "={eax}"(a),"={ebx}"(b),"={ecx}"(c),"={edx}"(d) : "{eax}"(eax), "{ecx}"(ecx) : "eax","ebx","ecx","edx")
    }

    cpu_name.eax3 = a;
    cpu_name.ebx3 = b;
    cpu_name.ecx3 = c;
    cpu_name.edx3 = d;

    return Some(cpu_name);
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct CpuName {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    eax2: u32,
    ebx2: u32,
    ecx2: u32,
    edx2: u32,
    eax3: u32,
    ebx3: u32,
    ecx3: u32,
    edx3: u32,
}

impl CpuName {
    /// Return cpu identification as human readable string.
    pub fn as_string<'a>(&'a self) -> &'a str {
        unsafe {
            let cpu_string_part = self as *const CpuName as *const u8;
            let slice = core::slice::from_raw_parts(cpu_string_part, 3 * 4 * 4);
            let byte_array: &'a [u8] = core::mem::transmute(slice);
            core::str::from_utf8_unchecked(byte_array)
        }
    }
}