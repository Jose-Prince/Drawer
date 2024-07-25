use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
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

pub fn render(framebuffer: &mut Framebuffer, file_path: &str) -> Result<()> {
    let maze = load_maze(file_path)?;
    let rows = maze.len();
    let cols = maze[0].len();
    
    let block_size = std::cmp::min(framebuffer.get_width() / cols, framebuffer.get_height() / rows);

    for row in 0..rows {
        for col in 0..cols {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    Ok(())
}
