const CHIP8_SCREEN_WIDTH: usize = 64;
const CHIP8_SCREEN_HEIGHT: usize = 32;

/// Represents the pixels of the 64x32 CHIP-8 display.
///
/// A pixel is drawn when equal to true. When rendering it, make sure to
/// scale it to improve visibility in modern screens. See the
/// examples provided for reference.
pub struct Screen {
    screen: Vec<bool>,
    /// How many pixels wide the display is (64 for CHIP-8)
    pub width: usize,
    /// How many pixel high the display is (32 for CHIP-8)
    pub height: usize,
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            screen: vec![false; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT],
            width: CHIP8_SCREEN_WIDTH,
            height: CHIP8_SCREEN_HEIGHT,
        }
    }
}

impl Screen {
    /// Clear all pixels in the screen
    pub fn clear_screen(&mut self) {
        self.screen.fill_with(|| false);
    }

    /// Get the state of the pixel at the provided coordinates.
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.screen[x + y * self.width]
    }

    /// Flip the state of the pixel at the provided coordinates
    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        let idx = x + y * self.width;
        self.screen[idx] = !self.screen[idx];
    }

    /// Set the pixel at the provided coordinates
    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.screen[x + y * self.width] = true;
    }

    /// Clear the pixel at the provided coordinates
    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        self.screen[x + y * self.width] = false;
    }
}
