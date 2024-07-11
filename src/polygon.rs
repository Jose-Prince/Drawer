use crate::framebuffer::Framebuffer;
use crate::line::Line;
use nalgebra_glm as glm;

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

            // Convertimos los valores a f64 y creamos glm::TVec3<f64>
            let vertex1 = glm::vec3(first_pos[0] as f64, first_pos[1] as f64, 0.0);
            let vertex2 = glm::vec3(last_pos[0] as f64, last_pos[1] as f64, 0.0);

            self.line(vertex1, vertex2);
        }
    }
}
