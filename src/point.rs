use crate::consts::SQUARE_SIZE;

#[derive(Clone, Copy, Debug, PartialEq)] // <-- Derive PartialEq
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

// case SDLK_UP:    if (snake.dy == 0) { snake.dx = 0; snake.dy = -SQUARE_SIZE; } break;
// case SDLK_DOWN:  if (snake.dy == 0) { snake.dx = 0; snake.dy = SQUARE_SIZE; } break;
// case SDLK_LEFT:  if (snake.dx == 0) { snake.dx = -SQUARE_SIZE; snake.dy = 0; } break;
// case SDLK_RIGHT: if (snake.dx == 0) { snake.dx = SQUARE_SIZE; snake.dy = 0; } break;
    pub fn move_in_direction(&mut self, direction: Direction) {
        let square_size = SQUARE_SIZE as i32;

        match direction {
            Direction::Up => self.y -= square_size,
            Direction::Down => self.y += square_size,
            Direction::Left => self.x -= square_size,
            Direction::Right => self.x += square_size,
        }
    }
}
