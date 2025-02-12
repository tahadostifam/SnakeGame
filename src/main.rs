use std::{path::Path, time::Duration};

mod consts;
mod draw;
mod food;
mod point;
mod snake;
mod text;

use consts::*;
use food::Food;
use point::Direction;
use sdl2::{event::Event, keyboard::Keycode};
use snake::{self_collisioned, wall_collisioned, Snake};
use text::render_text;

fn main() {
    let ctx = sdl2::init().unwrap();
    let ttf_ctx = sdl2::ttf::init().unwrap();
    let vsub = ctx.video().unwrap();

    let font_path = Path::new("assets/Carlito-Regular.ttf");
    let font = ttf_ctx.load_font(font_path, 20).unwrap();

    let window = vsub
        .window("SnakeGame", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = &mut window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = ctx.event_pump().unwrap();
    let mut direction = Direction::Right;
    let mut snake = Snake::new();
    let mut snake_speed = 10;
    let mut score: u32 = 0;
    let mut lost = false;

    let mut food = Food::new();

    'running: loop {
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();

        snake.draw_snake(canvas);
        food.draw_food(canvas);
        render_text(
            &format!("Score: {}", score),
            2,
            2,
            &font,
            &texture_creator,
            canvas,
        );
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

        if snake_speed >= 20 {
            snake_speed = 14;
        }

        // Eat food
        if new_head.x == food.x.try_into().unwrap() && new_head.y == food.y.try_into().unwrap() {
            snake_speed += 2;
            score += 1;
            food.set_new_position();
        } else if wall_collisioned(head) || self_collisioned(head, &mut snake.list) {
            break 'running;
        } else {
            snake.list.pop_back(); // Remove tail unless food is eaten
        }

        snake.list.push_front(new_head);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / snake_speed));
    }
}
