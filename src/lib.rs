//! Implements the VM that runs the CHIP-8 interpreter leaving you free to
//! focus on the frontend with the graphics library and renderer of your choice.
//!
//! # Example
//! This is a basic skeleton of how to start implementing a frontend. 
//! It's recommended to use the [anyhow] crate as well.
//! ```ignore
//! use schip8::Chip8; 
//! use anyhow::{Context, Result};
//!
//! fn main() -> Result<()> {
//!     let mut chip = Chip8::default();
//!     
//!     // The load_file function needs to be implemented
//!     let file = load_file("roms/TETRIS")?;    
//!     chip.load_rom(&file).context("Loading ROM file")?;
//!
//!     // Use the frontend to make this loop run at 60 Hz
//!     loop {
//!         // Process input
//!         // ...
//!
//!         chip.tick().context("Interpreter tick")?;
//!
//!         // Render screen - check the examples for scaling demo 
//!         for y in 0..chip.screen.height {
//!             for x in 0..chip.screen.width {
//!                 if chip.screen.get_pixel(x, y) {
//!                     // Draw
//!                 }
//!             }
//!         }
//!
//!         // Reset with chip.reset() if reset key is pressed
//!
//!         if chip.should_play_sound() {
//!             // Play a tone
//!         } 
//!     }
//! }
//! ```
//! An example frontend using Macroquad has been provided [here].
//!
//! [here]: https://github.com/overthemil/schip8-macroquad
//! [anyhow]: https://crates.io/crates/anyhow/

mod config;
mod cpu;
mod errors;
mod memory;
mod screen;

pub use config::Config;
pub use cpu::Cpu;
pub use errors::ChipError;
pub use screen::Screen;

const MEMORY_SIZE: usize = 4096;

/// Represents the CHIP-8 VM that acts as the interpreter.
pub struct Chip8 {
    /// The full memory of the machine.
    ///
    /// The whole range is readable and writable and acts as RAM. The font and rom
    /// data are loaded to specific regions in this memory, typically in the lower addresses.
    pub memory: [u8; MEMORY_SIZE],
    /// The display representing the pixels written to by the CPU.
    pub screen: Screen,
    /// Changes various settings of the interpreter. Ability to change the tick rate to
    /// improve the feel of certain ROMs if necessary.
    pub config: Config,
    /// The CPU containing the core of the interpreter.
    pub cpu: Cpu,
    rom: Vec<u8>,
}

impl Chip8 {
    /// Create a new CHIP-8 interpreter with a custom [Config].
    pub fn new(config: Config) -> Self {
        let mut c8 = Chip8 {
            config,
            ..Default::default()
        };
        c8.load_default_font();
        c8.cpu.pc = c8.config.rom_base_addr;

        c8
    }

    /// Performs a single Fetch-Decode-Execute cycle in the [Cpu].
    pub fn step(&mut self) -> Result<(), ChipError> {
        self.cpu.step(&mut self.memory, &mut self.screen)?;

        Ok(())
    }

    /// Execute a full render cycle. At 60fps, this should be executed 60 times per second.
    ///
    /// The amount of steps that occurs in each render cycle is determined by the tick rate.
    pub fn tick(&mut self) -> Result<(), ChipError> {
        for _ in 0..self.config.tick_rate {
            self.step()?;
        }

        if self.cpu.timer_delay > 0 {
            self.cpu.timer_delay -= 1;
        }
        if self.cpu.timer_sound > 0 {
            self.cpu.timer_sound -= 1;
        }

        Ok(())
    }

    /// Sets the machine as if newly created. Any changed configs and loaded ROMs persist.
    pub fn reset(&mut self) {
        self.screen.clear_screen();
        self.reset_memory();
        self.cpu.reset();
        self.cpu.pc = self.config.rom_base_addr;
    }

    /// Set any of the keys in the keypad (0x0 - 0xF) as pressed.
    pub fn set_input(&mut self, keys_pressed: [bool; 16]) {
        self.cpu.keypad = keys_pressed;
    }

    /// Announces if a tone should be played.
    pub fn should_play_sound(&self) -> bool {
        self.cpu.timer_sound > 0
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut c8 = Chip8 {
            memory: [0; MEMORY_SIZE],
            screen: Screen::default(),
            config: Config::default(),
            cpu: Cpu::default(),
            rom: Vec::new(),
        };
        c8.load_default_font();
        c8.cpu.pc = c8.config.rom_base_addr;

        c8
    }
}
