use std::collections::VecDeque;

use sdl2::{render::Canvas, video::Window};

use crate::{
    consts::{SNAKE_COLOR, SQUARE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH},
    draw::draw_square,
    point::Point,
};

pub struct Snake {
    pub list: VecDeque<Point>,
}

impl Snake {
    pub fn new() -> Self {
        let center_x = 3 * SQUARE_SIZE as i32;
        let center_y = 3 * SQUARE_SIZE as i32;
        let mut list: VecDeque<Point> = VecDeque::new();
        list.push_back(Point::new(center_x, center_y));
        Self { list }
    }

    pub fn draw_snake(&mut self, canvas: &mut Canvas<Window>) {
        for segment in &self.list {
            draw_square(canvas, SNAKE_COLOR, segment.x, segment.y);
        }
    }
}

pub fn wall_collisioned(head: Point) -> bool {
    head.x > WINDOW_WIDTH.try_into().unwrap()
        || head.x < 0
        || head.y > WINDOW_HEIGHT.try_into().unwrap()
        || head.y < 0
}

pub fn self_collisioned(head: Point, snake: &mut VecDeque<Point>) -> bool {
    for &segment in snake.clone().iter().skip(1) {
        if segment == head {
            return true;
        }
    }
    return false;
}
