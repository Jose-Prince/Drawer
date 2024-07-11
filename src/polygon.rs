use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn polygon(&mut self, arr: &Vec<[isize; 2]>);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, arr: &Vec<[isize; 2]>) {
        if arr.len() < 2 {
            return; // No se puede formar un polígono con menos de 2 puntos
        }

        for i in 0..arr.len() {
            let first_pos = arr[i];
            let last_pos = if i < arr.len() - 1 {
                arr[i + 1]
            } else {
                arr[0]
            };

            // Convertimos los valores a f64 antes de pasarlos a `line`
            let x0 = first_pos[0] as f64;
            let y0 = first_pos[1] as f64;
            let x1 = last_pos[0] as f64;
            let y1 = last_pos[1] as f64;

            self.line(x0, y0, x1, y1);
        }
    }
}
