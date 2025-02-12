use rand::random;
use sdl2::{render::Canvas, video::Window};

use crate::{
    consts::{FOOD_COLOR, SQUARE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH},
    draw::draw_square,
};

#[derive(Debug, Clone)]
pub struct Food {
    pub x: u32,
    pub y: u32,
}

impl Food {
    pub fn new() -> Self {
        let pos = Self::new_position();
        Self { x: pos.0, y: pos.1 }
    }

    pub fn draw_food(&mut self, canvas: &mut Canvas<Window>) {
        let pos = Self::new_position();
        draw_square(
            canvas,
            FOOD_COLOR,
            pos.0.try_into().unwrap(),
            pos.1.try_into().unwrap(),
        );
    }

    pub fn new_position() -> (u32, u32) {
        let x = (random::<u32>() % (WINDOW_WIDTH / SQUARE_SIZE)) * SQUARE_SIZE;
        let y = (random::<u32>() % (WINDOW_HEIGHT / SQUARE_SIZE)) * SQUARE_SIZE;
        (x, y)
    }
}
