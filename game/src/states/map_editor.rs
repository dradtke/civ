use allegro::{Color, Event};
use allegro_font::FontAlign;
use std::collections::HashSet;
use std::u8;

#[no_mangle]
#[derive(Clone)]
pub struct MapEditor {
    x_start: f32,
    y_start: f32,
    camera_speed: f32,
    pressed_keys: ::util::KeySet,
    map: ::MapDef,
}

impl MapEditor {
    pub fn new(map: ::MapDef) -> MapEditor {
        MapEditor{
            x_start: 50.0,
            y_start: 50.0,
            camera_speed: 5.0,
            pressed_keys: ::util::KeySet::new(),
            map: map,
        }
    }
}

impl ::GameState for MapEditor {
    fn update(&mut self, _: &::Platform) -> Option<Box<::GameState>> {
        if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Right) {
            self.x_start -= self.camera_speed;
        } else if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Left) {
            self.x_start += self.camera_speed;
        } else if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Down) {
            self.y_start -= self.camera_speed;
        } else if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Up) {
            self.y_start += self.camera_speed;
        }

        None
    }

    fn render(&self, p: &::Platform) {
        let white = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
        ::draw_map(p, &self.map, self.x_start, self.y_start);
        ::draw_text(p, white, 10.0, 10.0, FontAlign::Left, "Welcome to the map editor!");
    }

    fn handle_event(&mut self, _: &::Platform, event: Event) -> Option<Box<::GameState>> {
        self.pressed_keys.handle_key_event(event);
        if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Space) {
            Some(Box::new(::states::game::Game::new(Some(self.map.clone()))))
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<::GameState> {
        Box::new((*self).clone())
    }
}
