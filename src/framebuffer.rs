use crate::bmp::write_bmp_file;
use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0);
        let current_color = Color::new(255, 255, 255);

        let buffer = vec![background_color.to_hex(); width * height];

        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        let color_hex = self.background_color.to_hex();
        self.buffer.fill(color_hex);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (self.width * y as usize) + x as usize;
            self.buffer[index] = self.current_color.to_hex();
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_point(&self, x: isize, y: isize) -> Option<u32> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (self.width * y as usize) + x as usize;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn save_as_bmp(&self, file_path: &str) -> std::io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}
