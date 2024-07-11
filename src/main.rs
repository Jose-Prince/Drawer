mod color;
mod framebuffer;
mod bmp;
mod line;

use framebuffer::Framebuffer;
use color::Color;
use line::Line;

fn main() {
    let width = 800;
    let height = 800;
    let mut fb = Framebuffer::new(width, height);

    // Set the current color to red and draw a diagonal line
    fb.set_current_color(Color::new(255, 0, 0));
    for i in 0..width.min(height) {
        fb.point(i as isize, 0 as isize);
    }
    fb.clear();
    fb.set_current_color(Color::new(255,255,255));
    fb.line(300.0, 300.0, 300.0, 600.0);
    fb.line(300.0, 600.0, 600.0, 600.0);
    fb.line(600.0, 600.0, 600.0, 300.0);
    fb.line(600.0, 300.0, 300.0, 300.0);
    

    // Save the framebuffer to a BMP file
    if let Err(e) = fb.save_as_bmp("square.bmp") {
        eprintln!("Failed to write BMP file: {}", e);
    }

}
