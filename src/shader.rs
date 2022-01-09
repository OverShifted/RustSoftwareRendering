use crate::utils::*;
use crate::buffer::*;
use crate::rasterizer::*;

use glam::Vec3Swizzles;

type VertexShaderOut = Vec2;
pub trait Shader {
    type Vertex;
    // type VertexShaderOut: std::ops::Mul<real> + std::ops::Add;

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec3, VertexShaderOut);
    fn fragment(&self, varyings: &VertexShaderOut) -> Vec4;

    fn draw(&self, buffer: &mut Buffer, vertices: &[Self::Vertex]) {
        for triangle_vertices in vertices.chunks_exact(3) {
            let (p0, varyings0) = self.vertex(&triangle_vertices[0]);
            let (p1, varyings1) = self.vertex(&triangle_vertices[1]);
            let (p2, varyings2) = self.vertex(&triangle_vertices[2]);

            let mut raster_ranges = Vec::new();
            TriangleRasterizer.rasterize(
                (buffer.width, buffer.height),
                &[
                    p0.xy(), p1.xy(), p2.xy()
                ],
                &mut raster_ranges
            );

            for y in 0..buffer.height {
                let ((left, right), left_weights, right_weights) = raster_ranges[y as usize];

                for x in 0..buffer.width {

                    if (x as i32) >= left && (x as i32) < right {
                        let left_weights = Vec3::from(left_weights);
                        let right_weights = Vec3::from(right_weights);

                        let weights = left_weights.lerp(right_weights, (x as real - left as real) / (right as real - left as real));

                        buffer.set_pixel(x, y, &self.fragment(&(varyings0 * weights.x + varyings1 * weights.y + varyings2 * weights.z)).to_array().map(|f| f as f32)[0..3]).unwrap();
                    }

                }
            }
        }
    }
}
