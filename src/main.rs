mod color;
mod framebuffer;
mod bmp;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use color::Color;
use line::Line;
use polygon::Polygon;
use nalgebra_glm as glm;

fn main() {
    let width = 800;
    let height = 800;
    let mut fb = Framebuffer::new(width, height);

    let points: Vec<[isize; 2]> = vec![
        [100, 100],
        [400, 500],
        [700, 300],
    ];

    // Define los colores para el borde y el relleno
    let border_color = Color::new(255, 0, 0); // Rojo
    let fill_color = Color::new(0, 255, 0);   // Verde
    
    fb.polygon(&points, border_color, fill_color);

    // Save the framebuffer to a BMP file
    if let Err(e) = fb.save_as_bmp("vertex.bmp") {
        eprintln!("Failed to write BMP file: {}", e);
    }

}
