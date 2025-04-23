use sdl2::{rect::Rect, render::Canvas, video::Window};

pub trait Entity {
    fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>>;
    fn get_hitbox(&self) -> Rect;
}