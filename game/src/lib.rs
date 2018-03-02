extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;
extern crate allegro_primitives;

mod assets;
mod draw;
mod states;
mod util;

use allegro::Color;
use std::mem;
use std::sync::Arc;

pub trait GameState {
    fn init(&mut self, _: &Platform) {}
    fn update(&mut self, _: &Platform) -> Option<Box<GameState>> { None }
    fn render(&self, _: &Platform) {}
    fn handle_event(&mut self, _: &Platform, _: allegro::Event) -> Option<Box<GameState>> { None }
    fn clean_up(&mut self) {}

    fn box_clone(&self) -> Box<GameState>;
}

impl Clone for Box<GameState> {
    fn clone(&self) -> Box<GameState> { self.box_clone() }
}

#[no_mangle]
pub fn init(p: &Platform, state: &mut Box<GameState>) {
    state.init(&p);
}

#[no_mangle]
pub fn update(p: &Platform, state: &mut Box<GameState>) -> Option<Box<GameState>> {
    state.update(p)
}

#[no_mangle]
pub fn render(p: &Platform, state: &Box<GameState>) {
    p.core.clear_to_color(Color::from_rgb(0, 0, 0));
    state.render(p);
}

#[no_mangle]
pub fn handle_event(p: &Platform, state: &mut Box<GameState>, event: allegro::Event) -> Option<Box<GameState>> {
    state.handle_event(p, event)
}

#[no_mangle]
pub fn clean_up(mut state: Box<GameState>) {
    state.clean_up();
    mem::forget(state);
}

#[allow(dead_code)]
pub struct Platform {
    pub core: Arc<allegro::Core>,
    pub font_addon: allegro_font::FontAddon,
    pub image_addon: allegro_image::ImageAddon,
    pub primitives_addon: allegro_primitives::PrimitivesAddon,
}

pub fn new_state(p: &Platform) -> Box<GameState> {
    Box::new(states::loading::Loading::new(p))
}

type MapDef = Vec<Vec<i32>>;

type Pos = (i32, i32);
