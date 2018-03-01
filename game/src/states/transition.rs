use allegro::{Color};
use allegro_font::FontAlign;
use std::u8;

#[no_mangle]
#[derive(Clone)]
pub struct Transition {
    from: Box<::GameState>,
    to: Box<::GameState>,
    /// Number of frames to transition for.
    timer: i32,
}

impl Transition {
    pub fn new(from: Box<::GameState>, to: Box<::GameState>) -> Transition {
        Transition{
            from: from,
            to: to,
            timer: 60,
        }
    }
}

impl ::GameState for Transition {
    fn render(&self, p: &::Platform) {
        let white = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
        ::draw_text(p, white, 10.0, 10.0, FontAlign::Left, "Transitioning!");
    }

    fn update(&mut self, p: &::Platform) -> Option<Box<::GameState>> {
        self.timer -= 1;
        if self.timer <= 0 {
            Some(self.to.clone())
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<::GameState> {
        Box::new((*self).clone())
    }
}
