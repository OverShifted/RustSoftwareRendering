use crate::utils::{real, Remap};

pub struct Buffer {
    data: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub depth: usize
}

impl Buffer {
    pub fn new(width: usize, height: usize, depth: usize) -> Buffer {
        Buffer {
            data: vec![0.0; width * height * depth],
            width, height, depth
        }
    }

    pub fn clear(&mut self) {
        // self.data = vec![0.0; self.width * self.height * self.depth];

        for i in 0..self.data.len() {
            self.data[i] = f32::NEG_INFINITY;
        }
    }

    pub fn fill_window_buffer(&self, window_buffer: &mut [u32], depth: bool) -> Result<(), &'static str> {
        if self.depth < 3 {
            Err("Buffer's depth is less than 3!")
        } else {
            let depth_range = if depth {
                let (min, max) = self.data.chunks_exact(self.depth).map(|pixel| pixel[3]).fold((real::INFINITY, real::NEG_INFINITY), |(min, max), b| (min.min(b), max.max(b)));
                min..max
            } else {
                0.0..0.0
            };
            
            for (i, pixel) in self.data.chunks_exact(self.depth).enumerate() {
                let (r, g, b) = if depth {
                    (
                        pixel[3].remap(depth_range.start, depth_range.end, 0.0, 255.0) as u32,
                        pixel[3].remap(depth_range.start, depth_range.end, 0.0, 255.0) as u32,
                        pixel[3].remap(depth_range.start, depth_range.end, 0.0, 255.0) as u32
                    )
                } else {
                    (
                        (pixel[0].clamp(0.0, 1.0) * 255.0) as u32,
                        (pixel[1].clamp(0.0, 1.0) * 255.0) as u32,
                        (pixel[2].clamp(0.0, 1.0) * 255.0) as u32
                    )
                };

                window_buffer[i] = r << 16 | g << 8 | b;
            }

            Ok(())
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: &[f32]) -> Result<(), &'static str> {
        if self.depth != value.len() {
            Err("Buffer's pixel boundary will overflow!")
        } else {
            // TODO: Dont run fragment shader
            if self.data[(y * self.width + x) * self.depth + 3] < value[3] {
                for (i, &v) in value.iter().enumerate().take(self.depth) {
                    self.data[(y * self.width + x) * self.depth + i] = v;
                }
            }

            Ok(())
        }
    }
}
