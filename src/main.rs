use std::time::Duration;

use sdl2::{self, event::Event, keyboard::Keycode, pixels::Color};

const WINDOW_WIDTH: u32 = 550;
const WINDOW_HEIGHT: u32 = 550;
const FPS: u32 = 30;

fn main() {
    let ctx = sdl2::init().unwrap();
    let vsub = ctx.video().unwrap();

    let window = vsub
        .window("SnakeGame", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(24, 25, 41));

    let mut event_pump = ctx.event_pump().unwrap();

    'running: loop {
        canvas.clear();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::ESCAPE), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1000));
    }
}
