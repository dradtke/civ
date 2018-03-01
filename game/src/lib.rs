extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

mod assets;
mod states;
mod util;

use allegro::Color;
use allegro_font::{FontAlign, FontDrawing};
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
}

pub fn new_state(p: &Platform) -> Box<GameState> {
    Box::new(states::loading::Loading::new(p))
}

type MapDef = Vec<Vec<i32>>;

pub fn draw_map(p: &Platform, map: &MapDef, x_start: f32, y_start: f32) {
    let x_step = (assets::tile_width()/4.0) * 3.0;
    // let y_step = tiles.tile_height/2.0;
    for (xtile, row) in map.iter().enumerate() {
        for (ytile, id) in row.iter().enumerate() {
            let x = x_start + (xtile as f32)*x_step;
            let y = y_start + (ytile as f32)*(assets::tile_height()) + if xtile%2==1 { assets::tile_height()/2.0 } else { 0.0 };
            assets::draw_tile(&p.core, *id, x, y, None);
        }
    }
}

pub fn draw_text<'a>(p: &Platform, color: allegro::Color, x: f32, y: f32, align: FontAlign, s: &'a str) {
    let font = allegro_font::Font::new_builtin(&p.font_addon).unwrap();
    p.core.draw_text(&font, color, x, y, align, s);
}
