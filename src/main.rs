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

    let vertex1 = glm::vec3(100.0, 100.0, 0.0);
    let vertex2 = glm::vec3(400.0, 500.0, 0.0);
    let vertex3 = glm::vec3(700.0, 300.0, 0.0);

    fb.set_current_color(Color::new(255,0,255));
    fb.line(vertex1, vertex2);
    fb.line(vertex2, vertex3);
    fb.line(vertex3, vertex1);
    

    // Save the framebuffer to a BMP file
    if let Err(e) = fb.save_as_bmp("vertex.bmp") {
        eprintln!("Failed to write BMP file: {}", e);
    }

}
