extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::video::WindowSurfaceRef;

use std::time::Duration;
use std::time::Instant;

mod triangle;

use crate::triangle::*;

fn main() {
    let window_size = (800, 600);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", window_size.0.try_into().unwrap(), window_size.1.try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut triangle = Triangle{
        p0: Vector2{ x: 0.9, y: 0.1 },
        p1: Vector2{ x: 0.5, y: 0.3 },
        p2: Vector2{ x: 0.5, y: 0.9 }
    };

    let mut direction_x = ( 1.1, -1.0, -0.8);
    let mut direction_y = (-0.7,  0.3,  0.4);

    let mut i = 0;
    let mut delta = 0.0;

    'running: loop {
        let now = Instant::now();
        i = (i + 1) % 500;

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

        {
            let mut surface = window.surface(&event_pump).unwrap();

            

            // surface.fill_rect(Rect::new(x as i32, y as i32, 1, 1), color).unwrap();

            // surface.with_lock_mut(|pixels| pixels[(y * window_size.0 + x) * 3] = 255);
            surface.with_lock_mut(|pixels| {
                for x in 0..window_size.0.into() {
                    for y in 0..window_size.1.into() {

                        let (left, right, rcolor, lcolor) = raster_ranges[y as usize];

                        let color = if (x as i32) >= left && (x as i32) < right {
                            // Color::RGB(255, 255, 255)

                            let full = right - left;
                            let right_dist = right - x as i32;
                            let left_dist = x as i32 - left;

                            let color = (
                                (rcolor.0 as i32 * right_dist - lcolor.0 as i32 * left_dist) / full,
                                (rcolor.1 as i32 * right_dist - lcolor.1 as i32 * left_dist) / full,
                                (rcolor.2 as i32 * right_dist - lcolor.2 as i32 * left_dist) / full
                            );

                            Color::RGB(color.0 as u8, color.1 as u8, color.2 as u8)
                        } else {
                            Color::RGB(0, 0, 0)
                        };

                        pixels[((y * window_size.0 + x) * 4    ) as usize] = color.r;
                        pixels[((y * window_size.0 + x) * 4 + 1) as usize] = color.g;
                        pixels[((y * window_size.0 + x) * 4 + 2) as usize] = color.b;
                    }
                }
            });

            surface.finish().unwrap();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        delta = now.elapsed().as_secs_f64();
        if i == 0 {
            println!("{} fps", 1.0 / delta);
        }
    }
}
