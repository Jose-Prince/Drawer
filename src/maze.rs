use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::cast_ray::cast_ray; // Asegúrate de importar la función cast_ray correctamente
use crate::player::Player;
use nalgebra_glm::Vec2;
use std::io::Result;

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    match cell {
        '+' => framebuffer.set_current_color(Color::new(255, 0, 0)), // Paredes
        '|' | '-' => framebuffer.set_current_color(Color::new(255, 0, 0)), // Paredes
        'p' => framebuffer.set_current_color(Color::new(0, 0, 255)), // Punto de inicio
        'g' => framebuffer.set_current_color(Color::new(255, 255, 0)), // Meta
        ' ' => framebuffer.set_current_color(Color::new(255, 255, 255)), // Espacios vacíos
        _ => framebuffer.set_current_color(Color::new(0, 0, 0)), // Color por defecto para caracteres desconocidos
    }

    for y in 0..block_size {
        for x in 0..block_size {
            framebuffer.point((x0 + x) as isize, (y0 + y) as isize);
        }
    }
}

fn draw_player(framebuffer: &mut Framebuffer, player: &Player, block_size: usize) {
    let half_size = block_size / 4; // Tamaño del jugador

    framebuffer.set_current_color(Color::new(0, 255, 0)); // Color del jugador

    for y in -(half_size as isize)..=(half_size as isize) {
        for x in -(half_size as isize)..=(half_size as isize) {
            framebuffer.point((player.pos.x as isize + x), (player.pos.y as isize + y));
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer, file_path: &str) -> Result<()> {
    let maze = load_maze(file_path)?;
    let rows = maze.len();
    let cols = maze[0].len();

    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);

    let mut player_pos = Vec2::new(0.0, 0.0);

    for row in 0..rows {
        for col in 0..cols {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
            if maze[row][col] == 'p' {
                player_pos = Vec2::new((col * block_size) as f32 + (block_size / 2) as f32, (row * block_size) as f32 + (block_size / 2) as f32);
            }
        }
    }

    let player = Player::new(player_pos.x, player_pos.y, 0.0);

    // Dibujar al jugador
    draw_player(framebuffer, &player, block_size);

    // Llamar a cast_ray con un único ángulo
    let angle = 45.0_f32.to_radians(); // Por ejemplo, 45 grados convertido a radianes
    cast_ray(framebuffer, &maze, &player, angle, block_size);

    Ok(())
}
