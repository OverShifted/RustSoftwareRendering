use minifb::*;
// use noise::{NoiseFn, Perlin};
use glam::Vec4Swizzles;

mod buffer;
mod utils;
mod shader;
use crate::utils::*;
use crate::buffer::*;
use crate::shader::*;

use std::time::Instant;

struct SimpleShader {
    pub t: real,
    pub camera: Mat4
}

impl Shader for SimpleShader {
    type Vertex = (Vec3, Vec2);
    type VertexShaderOut = Vec2;

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec3, Vec2) {
        let (pos, tex_pos) = *vertex;

        let t = self.t * 2.0;
        let mat = self.camera * Mat4::from_rotation_y(t) * Mat4::from_rotation_x(t * 2.0);

        ((mat * Vec4::new(pos.x, pos.y, pos.z, 0.0)).xyz(), tex_pos)
    }

    fn fragment(&self, varyings: &Vec2) -> Vec4 {
        let total = (varyings.x * 10.0).floor() +
                    (varyings.y * 10.0).floor();
        let is_even = total % 2.0 == 0.0;
        let col1 = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let col2 = Vec4::new(1.0, 1.0, 1.0, 1.0);

        if is_even { col1 } else { col2 }
    }
}

fn main() {
    let window_size = (900, 900);
    let mut window_buffer: Vec<u32> = vec![0; window_size.0 * window_size.1];
    let mut frame_buffer = Buffer::new(window_size.0, window_size.1, 4);

    let mut window = Window::new(
        ":)",
        window_size.0,
        window_size.1,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut shader = SimpleShader{
        t: 0.0,
        camera: Mat4::orthographic_rh(
            -2.0, 2.0, -2.0, 2.0,
            -0.5, 0.5
        )
        // camera: Mat4::perspective_rh(
        //     (600.0 as real).to_radians(),
        //     window_size.0 as real / window_size.1 as real,
        //     0.01, 100.0
        // )
    };

    let mut i = 0;
    let mut delta = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        i = (i + 1) % 100;
        
        frame_buffer.clear();
        shader.draw(&mut frame_buffer, &[
            (Vec3::new(-0.5,  0.5,  0.5), Vec2::new(0.0, 0.0)),
            (Vec3::new(-0.5, -0.5,  0.5), Vec2::new(1.0, 0.0)),
            (Vec3::new( 0.5,  0.5,  0.5), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5,  0.5), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5,  0.5, -0.5), Vec2::new(0.0, 0.0)),
            (Vec3::new(-0.5, -0.5, -0.5), Vec2::new(1.0, 0.0)),
            (Vec3::new(-0.5,  0.5,  0.5), Vec2::new(0.0, 1.0)),
            (Vec3::new(-0.5, -0.5,  0.5), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5,  0.5,  0.5), Vec2::new(0.0, 0.0)),
            (Vec3::new( 0.5,  0.5,  0.5), Vec2::new(1.0, 0.0)),
            (Vec3::new(-0.5,  0.5, -0.5), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5,  0.5, -0.5), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5, -0.5,  0.5), Vec2::new(0.0, 0.0)),
            (Vec3::new( 0.5, -0.5,  0.5), Vec2::new(1.0, 0.0)),
            (Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5, -0.5), Vec2::new(1.0, 1.0)),

            (Vec3::new( 0.5,  0.5, -0.5), Vec2::new(0.0, 0.0)),
            (Vec3::new( 0.5, -0.5, -0.5), Vec2::new(1.0, 0.0)),
            (Vec3::new( 0.5,  0.5,  0.5), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5,  0.5), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5,  0.5, -0.5), Vec2::new(0.0, 0.0)),
            (Vec3::new(-0.5, -0.5, -0.5), Vec2::new(1.0, 0.0)),
            (Vec3::new( 0.5,  0.5, -0.5), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5, -0.5), Vec2::new(1.0, 1.0)),

            // (Vec3::new( -1.0,  0.5,  0.5), Vec2::new(0.0, 0.0)),
            // (Vec3::new( -0.9,  0.0,  0.5), Vec2::new(1.0, 0.0)),
            // (Vec3::new( -0.5,  -0.5,  0.5), Vec2::new(0.0, 1.0)),

        ], &[
            0, 1, 2,
            3, 1, 2,

            4, 5, 6,
            7, 5, 6,

            8, 9, 10,
            11, 9, 10,

            12, 13, 14,
            15, 13, 14,

            16, 17, 18,
            19, 17, 18,

            20, 21, 22,
            23, 21, 22,
        ]);

        frame_buffer.fill_window_buffer(&mut window_buffer, window.is_key_down(Key::D)).unwrap();
        window.update_with_buffer(&window_buffer, window_size.0, window_size.1).unwrap();

        if window.is_key_down(Key::Right) {
            shader.t += 0.5 * delta;
            shader.t = shader.t % (2.0 * 3.14);
        } else if window.is_key_down(Key::Left) {
            shader.t -= 0.5 * delta;
            shader.t = shader.t % (2.0 * 3.14);
        }

        // shader.t += 0.005;
        // shader.t = shader.t % (2.0 * 3.14);

        delta = now.elapsed().as_secs_f64();
        if i == 0 {
            println!("{} fps", 1.0 / delta);
        }
    }
}
