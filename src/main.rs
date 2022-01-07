extern crate minifb;
use minifb::*;

mod triangle;
use crate::triangle::*;

use std::time::Duration;
use std::time::Instant;

fn main() {
    let window_size = (800, 600);
    let mut buffer: Vec<u32> = vec![0; window_size.0 * window_size.1];

    let mut window = Window::new(
        ":)",
        window_size.0,
        window_size.1,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));

    let mut triangle = Triangle{
        p0: Vector2{ x: 0.5, y: 0.1 },
        p1: Vector2{ x: 0.1, y: 0.9 },
        p2: Vector2{ x: 0.9, y: 0.9 }
    };

    // let mut direction_x = ( 1.1, -1.0, -0.8);
    // let mut direction_y = (-0.7,  0.3,  0.4);

    let mut direction_x = (0.0, 0.0, 0.0);
    let mut direction_y = (0.0, 0.0, 0.0);

    let mut i = 0;
    let mut delta = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        i = (i + 1) % 100;

        triangle.p0.x += 0.5 * direction_x.0 * delta;
        triangle.p1.x += 0.5 * direction_x.1 * delta;
        triangle.p2.x += 0.5 * direction_x.2 * delta;

        if triangle.p0.x > 1.0 || triangle.p0.x < 0.0 {
            direction_x.0 *= -1.0;
            triangle.p0.x = triangle.p0.x.max(0.0).min(1.0);
        }
        if triangle.p1.x > 1.0 || triangle.p1.x < 0.0 {
            direction_x.1 *= -1.0;
            triangle.p1.x = triangle.p1.x.max(0.0).min(1.0);
        }
        if triangle.p2.x > 1.0 || triangle.p2.x < 0.0 {
            direction_x.2 *= -1.0;
            triangle.p2.x = triangle.p2.x.max(0.0).min(1.0);
        }

        triangle.p0.y += 0.5 * direction_y.0 * delta;
        triangle.p1.y += 0.5 * direction_y.1 * delta;
        triangle.p2.y += 0.5 * direction_y.2 * delta;

        if triangle.p0.y > 1.0 || triangle.p0.y < 0.0 {
            direction_y.0 *= -1.0;
            triangle.p0.y = triangle.p0.y.max(0.0).min(1.0);
        }
        if triangle.p1.y > 1.0 || triangle.p1.y < 0.0 {
            direction_y.1 *= -1.0;
            triangle.p1.y = triangle.p1.y.max(0.0).min(1.0);
        }
        if triangle.p2.y > 1.0 || triangle.p2.y < 0.0 {
            direction_y.2 *= -1.0;
            triangle.p2.y = triangle.p2.y.max(0.0).min(1.0);
        }

        let raster_ranges = triangle.generate_raster_ranges(window_size);

        for x in 0..window_size.0 {
            for y in 0..window_size.1 {

                let (left, right, right_color, left_color) = raster_ranges[y as usize];

                let color = if (x as i32) >= left && (x as i32) < right {
                    let full = right - left;
                    let right_dist = right - x as i32;
                    let left_dist = x as i32 - left;

                    let color = (
                        (right_color.0 as i32 * left_dist + left_color.0 as i32 * right_dist) / full,
                        (right_color.1 as i32 * left_dist + left_color.1 as i32 * right_dist) / full,
                        (right_color.2 as i32 * left_dist + left_color.2 as i32 * right_dist) / full
                    );

                    (color.0 << 16 | color.1 << 8 | color.2) as u32
                } else {
                    0u32
                };

                buffer[y * window_size.0 + x] = color;
            }
        }


        window.update_with_buffer(&buffer, window_size.0, window_size.1).unwrap();

        delta = now.elapsed().as_secs_f64();
        if i == 0 {
            println!("{} fps", 1.0 / delta);
        }
    }
}
