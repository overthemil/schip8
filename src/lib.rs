mod config;
mod errors;
mod memory;

pub use errors::ChipError;
pub use config::Config;

const MEMORY_SIZE: usize = 4096; 

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    config: Config
}

impl Chip8 {
    pub fn new(config: Config) -> Self {
        let mut c8 = Chip8 {
            memory: [0; MEMORY_SIZE],
            config
        };
        c8.load_default_font();

        c8
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut c8 = Chip8 {
            memory: [0; MEMORY_SIZE],
            config: Config::default(),
        };
        c8.load_default_font();

        c8
    }
}
