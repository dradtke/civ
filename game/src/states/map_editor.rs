use allegro::{Color, Event};
use allegro_font::FontAlign;
use std::collections::HashSet;
use std::u8;

#[no_mangle]
#[derive(Clone)]
pub struct MapEditor {
    camera: ::Pos,
    camera_speed: i32,
    pressed_keys: ::util::KeySet,
    map: ::MapDef,
    selected_tile: ::Pos,
}

impl MapEditor {
    pub fn new(map: ::MapDef) -> MapEditor {
        MapEditor{
            camera: (0, 0),
            camera_speed: 5,
            pressed_keys: ::util::KeySet::new(),
            map: map,
            selected_tile: (0, 0),
        }
    }
}

impl ::GameState for MapEditor {
    fn update(&mut self, _: &::Platform) -> Option<Box<::GameState>> {
        if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Right) {
            self.camera.0 -= self.camera_speed;
        } else if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Left) {
            self.camera.0 += self.camera_speed;
        } else if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Down) {
            self.camera.1 -= self.camera_speed;
        } else if self.pressed_keys.is_down(::allegro::keycodes::KeyCode::Up) {
            self.camera.1 += self.camera_speed;
        }

        None
    }

    fn render(&self, p: &::Platform) {
        let white = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
        ::draw::map(p, &self.map, self.camera);
        ::draw::tile_highlight(p, self.camera, self.selected_tile);
        ::draw::text(p, white, (10, 10), FontAlign::Left, "Welcome to the map editor!");
    }

    fn handle_event(&mut self, _: &::Platform, event: Event) -> Option<Box<::GameState>> {
        if let Some((keycode, is_down)) = self.pressed_keys.handle_key_event(event) {
            return if keycode == ::allegro::keycodes::KeyCode::Space && is_down {
                Some(Box::new(::states::game::Game::new(Some(self.map.clone()))))
            } else {
                None
            }
        }

        match event {
            ::allegro::Event::MouseButtonDown{ x, y, button, .. } if button == 1 => {
                if let Some(pos) = ::draw::clicked_tile(&self.map, self.camera, (x, y)) {
                    self.selected_tile = pos;
                }
            },
            _ => (),
        }
        None
    }

    fn box_clone(&self) -> Box<::GameState> {
        Box::new((*self).clone())
    }
}
