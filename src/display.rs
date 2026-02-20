use minifb::{Key, Window, WindowOptions};

const SCALE: usize = 20;
const WIDTH: usize = 64 * SCALE;
const HEIGHT: usize = 32 * SCALE;

pub struct Display {
    // display buffer
    display: [u8; 64 * 32],
    buffer: Vec<u32>,
    window: Window,
}

impl Display {
    pub fn new() -> Self {
        let window = Window::new(
            "Chip-8 by Hernani Samuel Diniz",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap();

        Display {
            display: [0; 64 * 32],
            buffer: vec![0u32; WIDTH * HEIGHT],
            window,
        }
    }

    // Render converts display to scaled version buffer and updates screen
    pub fn render(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                let color = if self.display[y * 64 + x] == 1 {
                    0xFFB000
                } else {
                    0x000000
                };
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        self.buffer[(y * SCALE + dy) * WIDTH + (x * SCALE + dx)] = color;
                    }
                }
            }
        }
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    // Function to say the screen state (open or not)
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    // Safe screen usage
    pub fn get_pixel(&self, index: usize) -> Result<u8, DisplayError> {
        if index >= 64 * 32 {
            Err(DisplayError::InvalidPixelAccess)
        } else {
            Ok(self.display[index])
        }
    }

    pub fn set_pixel(&mut self, index: usize, value: u8) -> Result<bool, DisplayError> {
        if index >= 64 * 32 {
            Err(DisplayError::InvalidPixelAccess)
        } else if value == 1 || value == 0 {
            self.display[index] = value;
            Ok(true)
        } else {
            Err(DisplayError::InvalidPixelValue)
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum DisplayError {
    InvalidPixelAccess,
    InvalidPixelValue,
}

#[cfg(test)]
mod tests {
    use super::*;
    // Testing screen safety
    #[test]
    fn test_screen_full_use() {
        let mut display = Display::new();
        // Changes every pixel to 1
        for i in 0..2048 {
            // 64*32 = 2048
            assert!(display.set_pixel(i, 1).is_ok());
        }
        // Check if limits are being checked
        assert!(display.set_pixel(2048, 1).is_err());
        assert!(display.set_pixel(0, 2).is_err());
        // Check if all pixels have the value of 1
        for i in 0..2048 {
            assert_eq!(display.get_pixel(i).unwrap(), 1);
        }
        // These 2 for loops set every pixel to 0 and check if it's really 0
        for i in 0..2048 {
            assert!(display.set_pixel(i, 0).is_ok());
        }
        for i in 0..2048 {
            assert_eq!(display.get_pixel(i).unwrap(), 0);
        }
    }
}
