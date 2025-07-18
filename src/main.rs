use minifb::*;
use image::GenericImageView;

mod buffer;
mod utils;
mod shader;
mod model;
mod obj_shader;
use crate::utils::*;
use crate::buffer::*;
use crate::shader::*;

use std::time::Instant;

#[allow(dead_code)]
struct SimpleShader {
    pub t: real,
    pub light: Vec3,
    pub camera: Mat4,
    pub img: image::DynamicImage
}

impl Shader for SimpleShader {
    type Vertex = (Vec3, Vec3, Vec2);
    type VertexShaderOut = (Vec3, Vec2);

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec4, Self::VertexShaderOut) {
        let (pos, normal, tex_pos) = *vertex;

        let t = self.t / 2.0;
        let mat = self.camera * Mat4::from_rotation_y(t) * Mat4::from_rotation_x(t * 2.0);

        (
            mat * Vec4::new(pos.x, pos.y, pos.z, 1.0),
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
    let monkey = model::load_obj("monkey.obj");

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

    let mut shader = obj_shader::ObjShader{
        t: 0.0,
        light: Vec3::new(0.5, 1.2, 0.8).normalize(),
        camera: Mat4::perspective_rh(
            (110.0 as real).to_radians(),
            window_size.0 as real / window_size.1 as real,
            0.01, 100.0
        ) * Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        // img: image::open("crate.jpg").unwrap()
    };

    let mut i = 0;
    let mut delta = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        i = (i + 1) % 100;
        
        frame_buffer.clear();
        for y in 0..frame_buffer.height {
            for x in 0..frame_buffer.width {
                let top = [0.5, 0.8, 1.0, -1.0];
                let bottom = [0.8, 1.0, 1.0, -1.0];

                let y_f = y as real / frame_buffer.height as real;
                
                frame_buffer.set_pixel(x, y, &[
                    top[0] * y_f + bottom[0] * (1.0 - y_f),
                    top[1] * y_f + bottom[1] * (1.0 - y_f),
                    top[2] * y_f + bottom[2] * (1.0 - y_f),
                    top[3] * y_f + bottom[3] * (1.0 - y_f),
                ]).unwrap();
            }
        }

        shader.draw(&mut frame_buffer, &monkey.vertices, &monkey.indices);

        frame_buffer.fill_window_buffer(&mut window_buffer, window.is_key_down(Key::D)).unwrap();
        window.update_with_buffer(&window_buffer, window_size.0, window_size.1).unwrap();

        // if window.is_key_down(Key::Right) {
        //     shader.t += 0.5 * delta;
        // } else if window.is_key_down(Key::Left) {
        //     shader.t -= 0.5 * delta;
        // }

        shader.t += 0.9 * delta;
        shader.t %= 4.0 * std::f32::consts::PI;

        delta = now.elapsed().as_secs_f32();
        if i == 0 {
            println!("{:.3} fps", 1.0 / delta);
        }
    }
}
