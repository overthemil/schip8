# Schip8
A library with the aim to provide a simple CHIP-8 interpreter backend that can integrate into any graphics library or renderer of your choosing.

# Examples 
- [Macroquad](https://github.com/overthemil/schip8-macroquad)

# Quickstart
This is a basic skeleton of how to start implementing a frontend. 
It's recommended to use the [anyhow](https://crates.io/crates/anyhow/) crate as well.
```rust
use schip8::Chip8; 
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut chip = Chip8::default();
    
    // The load_file function needs to be implemented
    let file = load_file("roms/TETRIS")?;    
    chip.load_rom(&file).context("Loading ROM file")?;

    // Use the frontend to make this loop run at 60 Hz
    loop {
        // Process input
        // ...

        chip.tick().context("Interpreter tick")?;

        // Render screen - check the examples for scaling demo 
        for y in 0..chip.screen.height {
            for x in 0..chip.screen.width {
                if chip.screen.get_pixel(x, y) {
                    // Draw
                }
            }
        }

        // Reset with chip.reset() if reset key is pressed

        if chip.should_play_sound() {
            // Play a tone
        } 
    }
}
```

# Features
- [x] CHIP-8
- [ ] Super-Chip
- [ ] Better debugging tools
