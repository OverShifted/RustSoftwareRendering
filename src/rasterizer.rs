use crate::utils::*;

pub trait Rasterizer {
    type Vertices;
    type RowOut;
    fn rasterize(&self, canvas_size: (usize, usize), vertices: &Self::Vertices, out: &mut Vec<Self::RowOut>);
}

// Based on `y = mx + b`
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub m: real,
    pub b: real
}

impl Line {
    pub fn new(p0: Vec2, p1: Vec2) -> Line {
        let mut p0 = p0;

        if (p0.x - p1.x).abs() < 0.000000001 {
            p0.x += 0.000000001;
        }

        if (p0.y - p1.y).abs() < 0.000000001 {
            p0.y += 0.000000001;
        }

        let m = (p0.y - p1.y) / (p0.x - p1.x);
        let b = p0.y - m * p0.x;

        Line{ m, b }
    }

    pub fn x(&self, y: real) -> real {
        (y - self.b) / self.m
    }
}

fn unlerp(a: Vec2, b: Vec2, input_y: real) -> real {
    (input_y - a.y) / (b.y - a.y)
}

pub struct TriangleRasterizer;
impl Rasterizer for TriangleRasterizer {

    type Vertices = [Vec2; 3];
    type RowOut = ((i32, i32), (real, real, real), (real, real, real));

    fn rasterize(&self, canvas_size: (usize, usize), vertices: &Self::Vertices, out: &mut Vec<Self::RowOut>) {

        struct VertexData {
            pub pos: Vec2,
            pub i: u32
        }

        let mut influences = [
            (0.0, 0.0),
            (0.0, 0.0),
            (0.0, 0.0)
        ];

        let p0 = VertexData{ pos: vertices[0], i: 0 };
        let p1 = VertexData{ pos: vertices[1], i: 1 };
        let p2 = VertexData{ pos: vertices[2], i: 2 };

        // Sort points by y
        let (p0, p1) = if p0.pos.y > p1.pos.y { (p1, p0) } else { (p0, p1) };
        let (p1, p2) = if p1.pos.y > p2.pos.y { (p2, p1) } else { (p1, p2) };
        let (p0, p1) = if p0.pos.y > p1.pos.y { (p1, p0) } else { (p0, p1) };

        let p0_p1 = Line::new(p0.pos, p1.pos);
        let p0_p2 = Line::new(p0.pos, p2.pos);
        let p1_p2 = Line::new(p1.pos, p2.pos);

        let p1_is_on_left = p0_p2.x(p1.pos.y) > p1.pos.x;

        for y_int in 0..canvas_size.1 {

            let y = (y_int as real).remap(0.0, canvas_size.1 as real, 1.0, -1.0) - (1.0 / canvas_size.1 as real);

            out.push(
                if y < p0.pos.y || y > p2.pos.y {
                    ((-1, -1), (0.0, 0.0, 0.0), (0.0, 0.0, 0.0))
                } else {
                    let full_side = p0_p2.x(y);
                    let partial_side = if y >= p1.pos.y { p1_p2 } else { p0_p1 }.x(y);

                    let p2_influence_full_side = unlerp(p0.pos, p2.pos, y);
                    let p0_influence_full_side = 1.0 - p2_influence_full_side;

                    let (
                        p0_influence_partial_side,
                        p1_influence_partial_side,
                        p2_influence_partial_side
                    ) = if y >= p1.pos.y {
                        let p2_influence = unlerp(p1.pos, p2.pos, y);
                        (0.0, 1.0 - p2_influence, p2_influence)
                    } else {
                        let p1_influence = unlerp(p0.pos, p1.pos, y);
                        (1.0 - p1_influence, p1_influence, 0.0)
                    };

                    influences[p0.i as usize] = (p0_influence_full_side, p0_influence_partial_side);
                    influences[p1.i as usize] = (0.0                   , p1_influence_partial_side);
                    influences[p2.i as usize] = (p2_influence_full_side, p2_influence_partial_side);

                    // Swap if p1_is_on_left
                    let (left, right) = if p1_is_on_left {
                        influences[0] = (influences[0].1, influences[0].0);
                        influences[1] = (influences[1].1, influences[1].0);
                        influences[2] = (influences[2].1, influences[2].0);

                        (partial_side, full_side)
                    } else {
                        (full_side, partial_side)
                    };

                    (
                        (
                            ((left + 1.0) * canvas_size.0 as real / 2.0).round() as i32,
                            ((right + 1.0) * canvas_size.0 as real / 2.0).round() as i32
                        ),
                        (influences[0].0, influences[1].0, influences[2].0),
                        (influences[0].1, influences[1].1, influences[2].1),
                    )
                }
            );
        }
    }
}
