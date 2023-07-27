pub struct Config {
    pub rom_base_addr: usize,
    pub font_base_addr: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rom_base_addr: 0x200,
            font_base_addr: 0x000,
        }
    }
}
