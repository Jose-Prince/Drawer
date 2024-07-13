mod color;
mod framebuffer;
mod bmp;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use color::Color;
use line::Line;
use polygon::Polygon;
use minifb::{Window, WindowOptions};
use std::time::Duration;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let close_delay = Duration::from_secs(10);

    let mut fb = Framebuffer::new(window_width, window_height);

    let mut window = match Window::new(
        "Rust Graphics - Framebuffer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ) {
        Ok(window) => window,
        Err(e) => {
            eprintln!("Failed to create window: {}", e);
            return;
        }
    };

    let points: Vec<[isize; 2]> = vec![
        [100, 100],
        [400, 500],
        [700, 300],
    ];

    // Define los colores para el borde y el relleno
    let border_color = Color::new(255, 0, 0); // Rojo
    let fill_color = Color::new(0, 255, 0);   // Verde
    
    fb.polygon(&points, border_color, fill_color);

    window.update_with_buffer(&fb.get_buffer(), window_width, window_height).unwrap();

    std::thread::sleep(close_delay);

    // Save the framebuffer to a BMP file
    // if let Err(e) = fb.save_as_bmp("polygon.bmp") {
    //     eprintln!("Failed to write BMP file: {}", e);
    // }
}
