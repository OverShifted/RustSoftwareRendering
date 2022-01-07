#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

// Based on `y = mx + b`
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub m: f64,
    pub b: f64
}

impl Line {
    pub fn new(p0: Vector2, p1: Vector2) -> Line {
        let mut p0 = p0;

        if (p0.x - p1.x).abs() < 0.0000001 {
            p0.x += 0.0000001;
        }

        let m = (p0.y - p1.y) / (p0.x - p1.x);
        let b = p0.y - m * p0.x;

        Line{ m, b }
    }

    pub fn x(&self, y: f64) -> f64 {
        if self.m.abs() < 0.0000001 {
            0.0
        } else {
            (y - self.b) / self.m
        }
    }
}

pub struct Triangle {
    pub p0: Vector2,
    pub p1: Vector2,
    pub p2: Vector2
}

impl Triangle {
    pub fn generate_raster_ranges(&self, canvas_size: (usize, usize)) -> Vec<(i32, i32, (u8, u8, u8), (u8, u8, u8))> {
        let p0 = self.p0;
        let p1 = self.p1;
        let p2 = self.p2;

        // Sort points by y
        let (p0, p1) = if p0.y > p1.y { (p1, p0) } else { (p0, p1) };
        let (p1, p2) = if p1.y > p2.y { (p2, p1) } else { (p1, p2) };
        let (p0, p1) = if p0.y > p1.y { (p1, p0) } else { (p0, p1) };

        let p0_p1 = Line::new(p0, p1);
        let p0_p2 = Line::new(p0, p2);
        let p1_p2 = Line::new(p1, p2);

        let p1_is_on_left = {
            let x = p0_p2.x(p1.y);
            x > p1.x
        };

        let mut out = Vec::new();

        for y_int in 0..canvas_size.1 {
            let y = y_int as f64 / canvas_size.1 as f64;

            out.push(
                if y < p0.y || y > p2.y {
                    (0, 0, (0, 0, 0), (0, 0, 0))
                } else {
                    let full_side = p0_p2.x(y);
                    let partial_side = if y > p1.y { p1_p2 } else { p0_p1 }.x(y);

                    // Swap if p1_is_on_left
                    let (left, right) = if p1_is_on_left { (partial_side, full_side) } else { (full_side, partial_side) };

                    let full_side_interp = {
                        let dist_p0 = y - self.p0.y;
                        let dist_p2 = self.p2.y - y;
                        let dist_full = self.p2.y - self.p0.y;

                        let p0_tint = 255.0;
                        let p2_tint = 0.0;

                        (p0_tint * dist_p2 + p2_tint * dist_p0) / dist_full
                    };

                    let partial_side_interp = if y > p1.y {
                        let dist_p1 = y - self.p1.y;
                        let dist_p2 = self.p2.y - y;
                        let dist_full = self.p2.y - self.p1.y;

                        let p1_tint = 255.0;
                        let p2_tint = 0.0;

                        (p1_tint * dist_p2 + p2_tint * dist_p1) / dist_full
                    } else {
                        let dist_p0 = y - self.p0.y;
                        let dist_p1 = self.p1.y - y;
                        let dist_full = self.p1.y - self.p0.y;

                        let p0_tint = 255.0;
                        let p1_tint = 0.0;

                        (p0_tint * dist_p1 + p1_tint * dist_p0) / dist_full
                    };

                    (
                        (left * canvas_size.0 as f64) as i32,
                        (right * canvas_size.0 as f64) as i32,

                        (color_left, 255 - color_left, color_left),
                        (color_right, 255 - color_right, color_right),
                        // (255, 0, 0),
                        // (0, 255, 0)
                    )
                }
            );
        }

        out
    }
}