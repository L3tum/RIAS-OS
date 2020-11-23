use pic8259_simple::ChainedPics;
use crate::config::DATA;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
// Hz
const PIT_BASE_FREQUENCY: u32 = 3579545 / 3;
const TICKS_PER_SECOND: u32 = DATA.time_interrupt_per_second;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn setup_pic() {
    unsafe {
        PICS.lock().initialize();

        use x86_64::instructions::port::Port;

        // PIT_CTL
        let mut pit_ctl: Port<u8> = Port::new(0x43 as u16);

        // TIMER0 | SQUARE WAVE MODE | WRITE WORD
        pit_ctl.write(0x00 | 0x06 | 0x30);

        const TIMER_RELOAD: u16 = (PIT_BASE_FREQUENCY / TICKS_PER_SECOND) as u16;

        // TIMER0_CTL
        let mut timer0_ctl: Port<u8> = Port::new(0x40 as u16);

        timer0_ctl.write(((TIMER_RELOAD & 0xff) as u8));
        timer0_ctl.write(((TIMER_RELOAD >> 8) & 0xff) as u8);
    };
}