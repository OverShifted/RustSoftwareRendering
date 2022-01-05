extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use std::time::Instant;

mod triangle;

use crate::triangle::*;

pub fn main() {
    let window_size = (1280, 720);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", window_size.0, window_size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut triangle = Triangle{
        p0: Vector2{ x: 0.9, y: 0.1 },
        p1: Vector2{ x: 0.5, y: 0.3 },
        p2: Vector2{ x: 0.5, y: 0.9 }
    };

    let mut direction_x = (1.1, -1.0, -0.8);
    let mut direction_y = (1.1, -1.0, -0.8);

    let mut i = 0;
    let mut delta = 0.0;

    'running: loop {
        let now = Instant::now();
        i = (i + 1) % 500;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

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

                if (x as i32) > raster_ranges[y as usize].0 && (x as i32) < raster_ranges[y as usize].1 {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }

                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }
        }

        // canvas.set_draw_color(Color::RGB(255, 255, 255));
        // for y in 0..window_size.1 {
        //     let (start, stop) = raster_ranges[y as usize];

        //     if start != stop {
        //         canvas.draw_line(Point::new(start, y as i32), Point::new(stop, y as i32)).unwrap();
        //     }
        // }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
        delta = now.elapsed().as_secs_f64();
        if i == 0 {
            println!("{}", delta);
        }
    }
}