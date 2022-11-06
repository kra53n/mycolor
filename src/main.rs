extern crate sdl2;

mod tools;
mod config;
mod colors_rect;
mod colors_line;

use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use crate::colors_rect::ColorsRect;
use crate::colors_line::ColorsLine;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    //let color_rect: Rect = Rect::new(20, 20, 20, 20);
    let mut colors_rect: ColorsRect = ColorsRect::new(Rect::new(20, 20, 400, 200));
    let mut colors_line: ColorsLine = ColorsLine::new(Rect::new(20, 236, 400, 8));

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(i, 64, 64));
        colors_rect.draw(&mut canvas);
        colors_line.draw(&mut canvas);

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
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
