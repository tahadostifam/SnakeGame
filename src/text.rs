use sdl2::{pixels::Color, rect::Rect, render::TextureCreator, ttf::Font};
use sdl2::video::WindowContext;

pub fn render_text(
    text: &str,
    x: i32,
    y: i32,
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    let surface = font
        .render(text)
        .blended(Color::RGB(255, 255, 255)) // White color
        .expect("Failed to create text surface");

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .expect("Failed to create text texture");

    let text_rect = Rect::new(x, y, surface.width(), surface.height());

    canvas.copy(&texture, None, text_rect).unwrap();
}
