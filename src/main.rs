mod color;
mod framebuffer;
mod bmp;

use framebuffer::Framebuffer;
use color::Color;

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
    for i in 0..width.min(height) {
        fb.point(i as isize, 10 as isize);
    }

    // Save the framebuffer to a BMP file
    if let Err(e) = fb.save_as_bmp("out.bmp") {
        eprintln!("Failed to write BMP file: {}", e);
    }
    
}
