pub struct Config {
    pub rom_base_addr: usize,
    pub tick_rate: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rom_base_addr: 0x200,
            tick_rate: 10,
        }
    }
}
