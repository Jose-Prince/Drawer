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

pub fn render(framebuffer: &mut Framebuffer, file_path: &str) -> Result<Vec<Vec<char>>> {
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

    Ok(maze)
}

pub fn is_wall(maze: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y < maze.len() && x < maze[0].len() {
        maze[y][x] == '+' || maze[y][x] == '|' || maze[y][x] == '-'
    } else {
        false
    }
}