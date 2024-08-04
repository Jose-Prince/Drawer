use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
) {
    let mut d = 0.0;

    framebuffer.set_current_color(Color::new(255, 0, 0));

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        // Asegúrate de que los índices i y j estén dentro de los límites del laberinto
        let i = x / block_size;
        let j = y / block_size;

        if j >= maze.len() || i >= maze[0].len() || maze[j][i] != ' ' {
            return;
        }

        framebuffer.point(x as isize, y as isize);

        d += 1.0;
    }
}
