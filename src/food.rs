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
        let mut instance = Self { x: 0, y: 0 };
        instance.set_new_position();
        instance
    }

    pub fn draw_food(&mut self, canvas: &mut Canvas<Window>) {
        draw_square(
            canvas,
            FOOD_COLOR,
            self.x.try_into().unwrap(),
            self.y.try_into().unwrap(),
        );
    }

    pub fn set_new_position(&mut self) {
        // food->x = (rand() % (SCREEN_WIDTH / SQUARE_SIZE)) * SQUARE_SIZE;
        self.x = (random::<u32>() % (WINDOW_WIDTH / SQUARE_SIZE)) * SQUARE_SIZE;
        self.y = (random::<u32>() % (WINDOW_HEIGHT / SQUARE_SIZE)) * SQUARE_SIZE;
    }
}
