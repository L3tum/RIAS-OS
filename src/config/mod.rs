pub struct Config {
    timer_interrupt_per_second: i32
}

pub const DATA: Config = include!(concat!(env!("OUT_DIR"), "/config/config_data.rs"));