mod config;
mod cpu;
mod errors;
mod memory;
mod screen;

pub use errors::ChipError;
pub use config::Config;
use cpu::Cpu;
use screen::Screen;

const MEMORY_SIZE: usize = 4096; 

pub struct Chip8 {
    pub memory: [u8; MEMORY_SIZE],
    pub screen: Screen,     
    pub config: Config,
    pub cpu: Cpu,
}

impl Chip8 {
    pub fn new(config: Config) -> Self {
        let mut c8 = Chip8 {
            config,
            ..Default::default()
        };
        c8.load_default_font();
        c8.cpu.pc = c8.config.rom_base_addr; 

        c8
    }

    pub fn step(&mut self) -> Result<(), ChipError> {
        self.cpu.step(&mut self.memory, &mut self.screen)?;

        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), ChipError> {
        for _ in 0..self.config.tick_rate {
            self.step()?;
        }

        Ok(())
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut c8 = Chip8 {
            memory: [0; MEMORY_SIZE],
            screen: Screen::default(),
            config: Config::default(),
            cpu: Cpu::default(),
        };
        c8.load_default_font();
        c8.cpu.pc = c8.config.rom_base_addr; 

        c8
    }
}
