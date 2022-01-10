use minifb::*;
use noise::{NoiseFn, Perlin};

mod buffer;
mod utils;
mod rasterizer;
mod shader;
use crate::utils::*;
use crate::buffer::*;
use crate::shader::*;

use std::time::Instant;

struct SimpleShader {
    pub perlin: Perlin,
    pub grad: colorgrad::Gradient
}

impl SimpleShader {
    fn get_color(&self, val: real) -> Vec4 {
        let sea_level = 0.2;
        let mountain_level = 0.5;
        let snow_level = 0.9;

        if val < sea_level {
            Vec4::new(0.0, 0.0, 1.0, 1.0)
        } else if val < mountain_level {
            Vec4::new(0.0, 1.0, 0.0, 1.0)
        } else if val < snow_level {
            Vec4::new(0.2, 0.2, 0.0, 1.0)
        } else {
            Vec4::new(1.0, 1.0, 1.0, 1.0)
        }
    }
}

impl Shader for SimpleShader {
    type Vertex = (Vec3, Vec2);
    type VertexShaderOut = Vec2;

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec3, Vec2) {
        *vertex
    }

    fn fragment(&self, varyings: &Vec2) -> Vec4 {
        // Vec4::new(varyings.x, varyings.y, 0.0, 1.0)
        let val = self.perlin.get(varyings.to_array().map(|v| v * 5.0)).remap(-1.0, 1.0, 0.0, 1.0);
        let val = self.grad.at(val);
        Vec4::new(val.r, val.g, val.b, val.a)
        // Vec4::new(val, val, val, 1.0)
        // let val = self.get_color(val);
        // Vec4::new(val, val, val, 1.0)
        // val
        // Vec4::new(varyings.x, varyings.y, 0.0, 1.0)
        // Vec4::new(weights.x, weights.y, weights.z, 1.0)
        // Vec4::new(1.0, 1.0, 1.0, 1.0)
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

    let shader = SimpleShader{
        perlin: Perlin::new(),
        grad: colorgrad::CustomGradient::new()
            .html_colors(&["blue", "green", "brown", "white"])
            .build().unwrap() 
    };

    let mut i = 0;
    let mut delta;

    // let mut x = 0.0;
    // let mut y = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        i = (i + 1) % 100;
        
        frame_buffer.clear();
        shader.draw(&mut frame_buffer, &[
            // Vec3::new( 0.0,  0.6, 0.0),
            // Vec3::new(-0.6, -0.6, 0.0),
            // Vec3::new( x, y, 0.0),
            // Vec3::new( -0.5, 0.5, 0.0)

            (Vec3::new(-1.0,  1.0, 0.0), Vec2::new(0.0, 0.0)),
            (Vec3::new(-1.0, -1.0, 0.0), Vec2::new(0.0, 1.0)),
            (Vec3::new( 1.0,  1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec3::new( 1.0, -1.0, 0.0), Vec2::new(1.0, 1.0)),

        ], &[
            0, 1, 2,
            3, 1, 2
        ]);

        // if window.is_key_down(Key::D) { x += 0.02 }
        // if window.is_key_down(Key::A) { x -= 0.02 }
        // if window.is_key_down(Key::W) { y += 0.02 }
        // if window.is_key_down(Key::S) { y -= 0.02 }

        frame_buffer.fill_window_buffer(&mut window_buffer).unwrap();
        window.update_with_buffer(&window_buffer, window_size.0, window_size.1).unwrap();

        delta = now.elapsed().as_secs_f64();
        if i == 0 {
            println!("{} fps", 1.0 / delta);
        }
    }
}
