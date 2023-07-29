/// Settings to modify the behaviour of the interpreter.
pub struct Config {
    /// The location in memory where the loaded ROM data starts.
    pub rom_base_addr: usize,
    /// How many CPU cycles occur before every frame render cycle.
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
