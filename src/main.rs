mod color;
mod framebuffer;
mod bmp;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use color::Color;
use line::Line;
use polygon::Polygon;

fn main() {
    let width = 800;
    let height = 800;
    let mut fb = Framebuffer::new(width, height);

    let points: Vec<[isize; 2]> = vec![
        [350, 100],
        [350, 500],
        [1000, 100],
    ];

    fb.set_current_color(Color::new(255,255,255));
    fb.polygon(&points);
    

    // Save the framebuffer to a BMP file
    if let Err(e) = fb.save_as_bmp("polygon.bmp") {
        eprintln!("Failed to write BMP file: {}", e);
    }

}
