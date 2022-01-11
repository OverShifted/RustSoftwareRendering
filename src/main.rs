use minifb::*;
// use noise::{NoiseFn, Perlin};
use glam::Vec4Swizzles;

mod buffer;
mod utils;
mod rasterizer;
mod shader;
use crate::utils::*;
use crate::buffer::*;
use crate::shader::*;

use std::time::Instant;

struct SimpleShader {
    pub t: real
}

impl Shader for SimpleShader {
    type Vertex = (Vec3, Vec2);
    type VertexShaderOut = Vec2;

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec3, Vec2) {
        let (pos, tex_pos) = *vertex;

        let mat = Mat4::from_rotation_z(self.t);

        ((mat * Vec4::new(pos.x, pos.y, pos.z, 0.0)).xyz(), tex_pos)
    }

    fn fragment(&self, varyings: &Vec2) -> Vec4 {
        Vec4::new(varyings.x, varyings.y, 0.0, 1.0)
    }
}

fn main() {
    let window_size = (900, 900);
    let mut window_buffer: Vec<u32> = vec![0; window_size.0 * window_size.1];
    let mut frame_buffer = Buffer::new(window_size.0, window_size.1, 3);

    let mut window = Window::new(
        ":)",
        window_size.0,
        window_size.1,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut shader = SimpleShader{ t: 0.0 };

    let mut i = 0;
    let mut delta;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        i = (i + 1) % 100;
        
        frame_buffer.clear();
        shader.draw(&mut frame_buffer, &[
            (Vec3::new(-0.5,  0.5, 0.0), Vec2::new(0.0, 0.0)),
            (Vec3::new(-0.5, -0.5, 0.0), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5,  0.5, 0.0), Vec2::new(1.0, 0.0)),
            (Vec3::new( 0.5, -0.5, 0.0), Vec2::new(1.0, 1.0)),

        ], &[
            0, 1, 2,
            3, 1, 2
        ]);

        frame_buffer.fill_window_buffer(&mut window_buffer).unwrap();
        window.update_with_buffer(&window_buffer, window_size.0, window_size.1).unwrap();

        delta = now.elapsed().as_secs_f64();
        if i == 0 {
            println!("{} fps", 1.0 / delta);
        }

        shader.t += 0.01;
    }
}
