const CHIP8_SCREEN_WIDTH: usize = 64;
const CHIP8_SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    screen: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            screen: vec!(false; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT),
            width: CHIP8_SCREEN_WIDTH,
            height: CHIP8_SCREEN_HEIGHT,
        }
    }
}

impl Screen {
    pub fn clear_screen(&mut self) {
        self.screen.fill_with(|| false);
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.screen[x + y * self.width]
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        let idx = x + y * self.width;
        self.screen[idx] = !self.screen[idx];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.screen[x + y * self.width] = true;
    }

    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        self.screen[x + y * self.width] = false;
    }

    pub fn get_buffer(&self, scale: usize) -> Vec<Vec<bool>> {
        let buffer_width = self.width * scale;
        let buffer_height = self.height * scale;
        let mut buffer = vec![vec![false; buffer_width]; buffer_height];

        for y in 0..self.height {
            for dy in 0..scale {
                for x in 0..self.height {
                   for dx in 0..scale {
                        buffer[x + dx][y + dy] = self.get_pixel(x, y);
                   } 
                }
            }
        }

        buffer
    }
}
