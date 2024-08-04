use nalgebra_glm::Vec2;
use minifb::{Window, Key};
use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::color::Color;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32) -> Self {
        Player {
            pos: Vec2::new(x, y),
            a,
        }
    }

    pub fn process_events(&mut self, window: &Window) {
        const MOVE_SPEED: f32 = 10.0;
        const ROTATION_SPEED: f32 = PI / 10.0;

        if window.is_key_down(Key::Left) {
            self.a -= ROTATION_SPEED;
        }
        if window.is_key_down(Key::Right) {
            self.a += ROTATION_SPEED;
        }
        if window.is_key_down(Key::Up) {
            self.pos.x += self.a.cos() * MOVE_SPEED;
            self.pos.y += self.a.sin() * MOVE_SPEED;
        }
        if window.is_key_down(Key::Down) {
            self.pos.x -= self.a.cos() * MOVE_SPEED;
            self.pos.y -= self.a.sin() * MOVE_SPEED;
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        const PLAYER_SIZE: usize = 5;
        framebuffer.set_current_color(Color::new(0, 255, 0));

        for y in -(PLAYER_SIZE as isize)..=(PLAYER_SIZE as isize) {
            for x in -(PLAYER_SIZE as isize)..=(PLAYER_SIZE as isize) {
                framebuffer.point((self.pos.x as isize + x), (self.pos.y as isize + y));
            }
        }
    }
}
