use core::ops::{AddAssign, BitAndAssign, Deref};

use lazy_static::lazy_static;
use pc_keyboard::HandleControl;
use x86_64::{
    structures::{
        idt::{
            InterruptDescriptorTable,
            InterruptStackFrame,
            PageFaultErrorCode,
        }
    }
};

use crate::{
    arch::rias_x86_64::boot::gdt,
    arch::rias_x86_64::boot::pic,
    print,
    println,
};
use crate::config::DATA;

static TICKS_CURRENT_SECOND: spin::Mutex<u32> = spin::Mutex::new(0);
static SECONDS_SINCE_BOOT: spin::Mutex<u64> = spin::Mutex::new(0);
static TICKS_SINCE_BOOT: spin::Mutex<u64> = spin::Mutex::new(0);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = pic::PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn setup_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        IDT.page_fault.set_handler_fn(page_fault_handler);
        IDT[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        IDT[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        let idt: &'static InterruptDescriptorTable = &IDT;

        idt.load();
    }
}

fn eoi(index: u8) {
    unsafe {
        pic::PICS.lock()
            .notify_end_of_interrupt(index);
    }
}

// Handlers

// 3
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// 8
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64)
    -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

// 14
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    panic!();
}

// 32
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame)
{
    eoi(InterruptIndex::Timer.as_u8());

    TICKS_CURRENT_SECOND.lock().add_assign(1);
    TICKS_SINCE_BOOT.lock().add_assign(1);

    if TICKS_CURRENT_SECOND.lock().ge(&DATA.time_interrupt_per_second) {
        SECONDS_SINCE_BOOT.lock().add_assign(1);
        TICKS_CURRENT_SECOND.lock().bitand_assign(0);
    }
}

// 33
extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame)
{
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    eoi(InterruptIndex::Keyboard.as_u8());
}