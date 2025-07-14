use glam::Vec4Swizzles;
use image::GenericImageView;

use crate::{shader::Shader, utils::{real, *}};

pub struct ObjShader {
    pub t: real,
    pub light: Vec3,
    pub camera: Mat4,
    pub img: image::DynamicImage
}

impl Shader for ObjShader {
    type Vertex = obj::Vertex;
    type VertexShaderOut = (Vec3, Vec2);

    fn vertex(&self, vertex: &Self::Vertex) -> (Vec4, Self::VertexShaderOut) {
        let pos = Vec3::from_slice(&vertex.position) * 0.8;
        let normal = Vec3::from_slice(&vertex.normal).normalize();

        let t = self.t / 2.0;
        let mat = self.camera * Mat4::from_rotation_y(t) * Mat4::from_rotation_x(t * 2.0);

        (
            mat * Vec4::new(pos.x, pos.y, pos.z, 1.0),
            ((mat * Vec4::new(normal.x, normal.y, normal.z, 1.0)).xyz(), Vec2::ZERO)
        )
    }

    fn fragment(&self, varyings: &Self::VertexShaderOut) -> Vec4 {
        let (normal, tex_pos) = *varyings;

        // let pixel = self.img.get_pixel(
        //     (tex_pos.x * (self.img.width() as real - 0.001)) as u32,
        //     (tex_pos.y * (self.img.height() as real - 0.001)) as u32
        // ).0;

		let pixel = [94, 81, 57, 255];

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
