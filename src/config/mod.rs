include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub struct Config {
    pub time_interrupt_per_second: u32
}

pub const DATA: Config = Config {
    // every 10ms by default
    time_interrupt_per_second: TIME_INTERRUPT_PER_SECOND
};