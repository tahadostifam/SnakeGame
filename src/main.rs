use std::{ops::Deref, thread::sleep, time::Duration};

mod consts;
mod draw;
mod food;
mod point;
mod snake;

use consts::*;
use food::Food;
use point::Direction;
use sdl2::{event::Event, keyboard::Keycode};
use snake::{collisioned, Snake};

fn main() {
    let ctx = sdl2::init().unwrap();
    let vsub = ctx.video().unwrap();

    let window = vsub
        .window("SnakeGame", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = &mut window.into_canvas().build().unwrap();
    let mut event_pump = ctx.event_pump().unwrap();
    let mut direction = Direction::Right;
    let mut snake = Snake::new();
    let mut score: u32 = 0;
    
    let mut food = Food::new();
    food.draw_food(canvas);

    'running: loop {
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();

        snake.draw_snake(canvas);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } if direction != Direction::Down => {
                    direction = Direction::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } if direction != Direction::Up => {
                    direction = Direction::Down;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } if direction != Direction::Right => {
                    direction = Direction::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } if direction != Direction::Left => {
                    direction = Direction::Right;
                }
                _ => {}
            }
        }

        let head = snake.list.front().unwrap().clone();
        let mut new_head = head;
        new_head.move_in_direction(direction);

        // Check collision
        // if collisioned(new_head) {
        //     break 'running;
        // }

        if new_head.x == food.x.try_into().unwrap() && new_head.y == food.y.try_into().unwrap() {
            score += 1;
            food.draw_food(canvas);
        } else {
            snake.list.pop_back(); // Remove tail unless food is eaten
        }

        snake.list.push_front(new_head);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 200));
    }
}
