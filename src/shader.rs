use crate::utils::*;
use crate::buffer::*;

use glam::Vec3Swizzles;

struct Edge {
    curr_x: real,
    x_step: real,

    curr_ratio: real,
    ratio_step: real,

    first: bool,
}

impl Edge {
    pub fn new(min_y_vert: Vec2, max_y_vert: Vec2) -> Edge {
        let y_dist = max_y_vert.y - min_y_vert.y;
        let x_dist = max_y_vert.x - min_y_vert.x;

        Edge {
            curr_x: min_y_vert.x,
            x_step: x_dist / y_dist,

            curr_ratio: 0.0,
            ratio_step: 1.0 / y_dist,

            first: true
        }
    }

    pub fn next(&mut self) -> (real, real) {
        if self.first {
            self.first = false;
        } else {
            self.curr_x += self.x_step;
            self.curr_ratio += self.ratio_step;
        }
        (self.curr_x, self.curr_ratio)
    }
}

fn lerp(start: Vec3, stop: Vec3, t: real) -> Vec3 {
    (1.0 - t) * start + t * stop
}

pub trait Shader {
    type Vertex;
    type VertexShaderOut: Interpolate;

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec3, Self::VertexShaderOut);
    fn fragment(&self, varyings: &Self::VertexShaderOut) -> Vec4;

    fn draw(&self, buffer: &mut Buffer, vertices: &[Self::Vertex], indices: &[usize]) {
        for triangle_indices in indices.chunks_exact(3) {
            let (p0, varyings0) = self.vertex(&vertices[triangle_indices[0]]);
            let (p1, varyings1) = self.vertex(&vertices[triangle_indices[1]]);
            let (p2, varyings2) = self.vertex(&vertices[triangle_indices[2]]);

            let p0_unsorted = p0;
            let p1_unsorted = p1;
            let p2_unsorted = p2;

            let p0 = (Vec3::new(
                p0.x.remap(-1.0, 1.0, -0.5, buffer.width as real + 0.5).ceil(),
                p0.y.remap(1.0, -1.0, -0.5, buffer.height as real + 0.5).ceil(),
                p0.z
            ), 0);

            let p1 = (Vec3::new(
                p1.x.remap(-1.0, 1.0, -0.5, buffer.width as real + 0.5).ceil(),
                p1.y.remap(1.0, -1.0, -0.5, buffer.height as real + 0.5).ceil(),
                p1.z
            ), 1);

            let p2 = (Vec3::new(
                p2.x.remap(-1.0, 1.0, -0.5, buffer.width as real + 0.5).ceil(),
                p2.y.remap(1.0, -1.0, -0.5, buffer.height as real + 0.5).ceil(),
                p2.z
            ), 2);

            let (p0, p1) = if p0.0.y > p1.0.y { (p1, p0) } else { (p0, p1) };
            let (p1, p2) = if p1.0.y > p2.0.y { (p2, p1) } else { (p1, p2) };
            let (p0, p1) = if p0.0.y > p1.0.y { (p1, p0) } else { (p0, p1) };

            let mut edge_full = Edge::new(p0.0.xy(), p2.0.xy());
            let mut edge_up_half = Edge::new(p0.0.xy(), p1.0.xy());
            let mut edge_down_half = Edge::new(p1.0.xy(), p2.0.xy());

            for y in (p0.0.y as i32)..(p2.0.y as i32) {

                let mut influences = [
                    (0.0, 0.0),
                    (0.0, 0.0),
                    (0.0, 0.0)
                ];

                let mut full = edge_full.next();
                let mut half = if y <= p1.0.y as i32 {
                    edge_up_half.next()
                } else {
                    edge_down_half.next()
                };

                full.0 = full.0.ceil();
                half.0 = half.0.ceil();

                influences[p0.1].0 = if y <= p1.0.y as i32 { 1.0 - half.1 } else { 0.0 };
                influences[p1.1].0 = if y <= p1.0.y as i32 { half.1 } else { 1.0 - half.1 };
                influences[p2.1].0 = if y <= p1.0.y as i32 { 0.0 } else { half.1 };

                influences[p0.1].1 = 1.0 - full.1;
                influences[p1.1].1 = 0.0;
                influences[p2.1].1 = full.1;

                let (min, max) = if full.0 > half.0 {
                    (half.0, full.0)
                } else {
                    influences[p0.1] = (influences[p0.1].1, influences[p0.1].0);
                    influences[p1.1] = (influences[p1.1].1, influences[p1.1].0);
                    influences[p2.1] = (influences[p2.1].1, influences[p2.1].0);

                    (full.0, half.0)
                };

                let mut x_ratio = 0.0;
                let x_ratio_step = 1.0 / (max - min);
                for x in (min as i32)..(max as i32) {
                    let left_weights = Vec3::new(influences[0].0, influences[1].0, influences[2].0);
                    let right_weights = Vec3::new(influences[0].1, influences[1].1, influences[2].1);
                    let weights = lerp(left_weights, right_weights, x_ratio);

                    let fragment_colors = self.fragment(&Self::VertexShaderOut::interpolate(&varyings0, &varyings1, &varyings2, &weights)) * 255.0;
                    let fragment = [
                        fragment_colors[0] as f32,
                        fragment_colors[1] as f32,
                        fragment_colors[2] as f32,
                        real::interpolate(&p0_unsorted.z, &p1_unsorted.z, &p2_unsorted.z, &weights) as f32
                    ];

                    buffer.set_pixel(x as usize, y as usize, &fragment).unwrap();

                    x_ratio += x_ratio_step;
                }
            }
        }
    }
}
