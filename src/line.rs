use crate::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64);
}

impl Line for Framebuffer {
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let x1 = x1.round() as isize;
        let y1 = y1.round() as isize;
        let x2 = x2.round() as isize;
        let y2 = y2.round() as isize;

        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x, y);

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                if x == x2 {
                    break;
                }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y2 {
                    break;
                }
                err += dx;
                y += sy;
            }
        }
    }
}


