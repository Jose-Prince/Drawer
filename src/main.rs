mod framebuffer;
mod color;
mod fileReader;
mod bmp;
mod maze;
mod player;
mod cast_ray;

use framebuffer::Framebuffer;
use color::Color;
use player::Player;
use maze::render;

fn main() {
    let width = 1000;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);

    match render(&mut framebuffer, "src/maze.txt") {
        Ok(()) => println!("Maze rendered successfully"),
        Err(e) => eprintln!("Error rendering maze: {}", e),
    }

    if let Err(e) = framebuffer.save_as_bmp("output.bmp") {
        eprintln!("Error saving BMP file: {}", e);
    }
}
