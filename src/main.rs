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
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;

fn main() {
    let width = 1000;
    let height = 800;
    let mut framebuffer = Framebuffer::new(width, height);

    let mut window = Window::new(
        "Maze",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let file_path = "src/maze.txt";
    let maze = render(&mut framebuffer, file_path).unwrap();
    let mut player = Player::new(500.0, 400.0, 0.0); // Posición inicial del jugador

    while window.is_open() && !window.is_key_down(Key::Escape) {
        player.process_events(&window);

        // Redibujar el framebuffer
        framebuffer.clear();
        render(&mut framebuffer, file_path).unwrap();
        player.draw(&mut framebuffer);
        let angle = 45.0_f32.to_radians();
        cast_ray::cast_ray(&mut framebuffer, &maze, &player, angle, 10);

        window.update_with_buffer(&framebuffer.get_buffer(), width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }
}
