use minifb::*;
// use noise::{NoiseFn, Perlin};
use glam::Vec4Swizzles;
use image::GenericImageView;

mod buffer;
mod utils;
mod shader;
use crate::utils::*;
use crate::buffer::*;
use crate::shader::*;

use std::time::Instant;

struct SimpleShader {
    pub t: real,
    pub light: Vec3,
    pub camera: Mat4,
    pub img: image::DynamicImage
}

impl Shader for SimpleShader {
    type Vertex = (Vec3, Vec3, Vec2);
    type VertexShaderOut = (Vec3, Vec2);

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec3, Self::VertexShaderOut) {
        let (pos, normal, tex_pos) = *vertex;

        let t = self.t * 2.0;
        let mat = self.camera * Mat4::from_rotation_y(t) * Mat4::from_rotation_x(t * 2.0);

        (
            (mat * Vec4::new(pos.x, pos.y, pos.z, 1.0)).xyz(),
            (normal, tex_pos)
        )
    }

    fn fragment(&self, varyings: &Self::VertexShaderOut) -> Vec4 {
        let (normal, tex_pos) = *varyings;

        let pixel = self.img.get_pixel(
            (tex_pos.x * (self.img.width() as real - 0.001)) as u32,
            (tex_pos.y * (self.img.height() as real - 0.001)) as u32
        ).0;

        let pixel = Vec4::new(
            pixel[0] as real,
            pixel[1] as real,
            pixel[2] as real,
            pixel[3] as real,
        ) / 255.0;

        let light = normal.dot(self.light).max(0.0) + 0.4;
        pixel * light
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
        light: Vec3::new(0.5, 1.2, 0.8).normalize(),
        camera: Mat4::perspective_rh(
            (100.0 as real).to_radians(),
            window_size.0 as real / window_size.1 as real,
            0.01, 100.0
        ) * Mat4::from_scale_rotation_translation(
            Vec3::ONE,
            Quat::IDENTITY,
            // TODO: Z element dose not work.
            // Vec3::new(1.0, 1.0, 10.0)
            Vec3::ZERO
        ).inverse(),
        img: image::open("crate.jpg").unwrap()
    };

    let mut i = 0;
    let mut delta = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        i = (i + 1) % 100;
        
        frame_buffer.clear();
        shader.draw(&mut frame_buffer, &[
            (Vec3::new(-0.5,  0.5,  0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)),
            (Vec3::new(-0.5, -0.5,  0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)),
            (Vec3::new( 0.5,  0.5,  0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5,  0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5,  0.5, -0.5), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(0.0, 0.0)),
            (Vec3::new(-0.5, -0.5, -0.5), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec3::new(-0.5,  0.5,  0.5), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(0.0, 1.0)),
            (Vec3::new(-0.5, -0.5,  0.5), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5,  0.5,  0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 0.0)),
            (Vec3::new( 0.5,  0.5,  0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec3::new(-0.5,  0.5, -0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5,  0.5, -0.5), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5, -0.5,  0.5), Vec3::new(0.0, -1.0, 0.0), Vec2::new(0.0, 0.0)),
            (Vec3::new( 0.5, -0.5,  0.5), Vec3::new(0.0, -1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.0, -1.0, 0.0), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5, -0.5), Vec3::new(0.0, -1.0, 0.0), Vec2::new(1.0, 1.0)),

            (Vec3::new( 0.5,  0.5, -0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(0.0, 0.0)),
            (Vec3::new( 0.5, -0.5, -0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec3::new( 0.5,  0.5,  0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5,  0.5), Vec3::new(1.0, 0.0, 0.0), Vec2::new(1.0, 1.0)),

            (Vec3::new(-0.5,  0.5, -0.5), Vec3::new(0.0, 0.0, -1.0), Vec2::new(0.0, 0.0)),
            (Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.0, 0.0, -1.0), Vec2::new(1.0, 0.0)),
            (Vec3::new( 0.5,  0.5, -0.5), Vec3::new(0.0, 0.0, -1.0), Vec2::new(0.0, 1.0)),
            (Vec3::new( 0.5, -0.5, -0.5), Vec3::new(0.0, 0.0, -1.0), Vec2::new(1.0, 1.0)),

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

        // if window.is_key_down(Key::Right) {
        //     shader.t += 0.5 * delta;
        // } else if window.is_key_down(Key::Left) {
        //     shader.t -= 0.5 * delta;
        // }

        shader.t += 0.1 * delta;
        shader.t = shader.t % (2.0 * 3.14);

        delta = now.elapsed().as_secs_f64();
        if i == 0 {
            println!("{} fps", 1.0 / delta);
        }
    }
}
