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
    // Only works if points are sorted by `y`
    pub fn generate_raster_ranges(&self, canvas_size: (u32, u32)) -> Vec<(i32, i32)> {
        let p0 = self.p0;
        let p1 = self.p1;
        let p2 = self.p2;

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

        // println!("{:?}", p0_p2);

        let mut out = Vec::new();

        for y_int in 0..canvas_size.1 {
            let y = y_int as f64 / canvas_size.1 as f64;

            out.push(
                if y < p0.y || y > p2.y {
                    (0, 0)
                } else {
                    let left = p0_p2.x(y);
                    let right = if y > p1.y { p1_p2 } else { p0_p1 }.x(y);

                    // Swap if p1_is_on_left
                    let (left, right) = if p1_is_on_left { (right, left) } else { (left, right) };

                    (
                        (left * canvas_size.0 as f64) as i32,
                        (right * canvas_size.0 as f64) as i32
                    )
                }
            );
        }

        out
    }
}