use crate::utils::*;
use crate::buffer::*;
use crate::rasterizer::*;

use glam::Vec3Swizzles;

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
                        let interpolated = Self::VertexShaderOut::interpolate(&varyings0, &varyings1, &varyings2, &weights);
                        buffer.set_pixel(x, y, &self.fragment(&interpolated).to_array().map(|f| f as f32)[0..3]).unwrap();
                    }

                }
            }
        }
    }
}
