use crate::consts::SQUARE_SIZE;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub fn draw_square(canvas: &mut Canvas<Window>, color: Color, x: i32, y: i32) {
    canvas.set_draw_color(color);
    let square = Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE);
    canvas.fill_rect(square).unwrap();
}
