use allegro::{Color, Event};
use allegro_font::FontAlign;
use std::u8;

#[no_mangle]
#[derive(Clone)]
pub struct MapEditor {
    camera: ::Pos,
    camera_speed: i32,
    pressed_keys: ::util::KeySet,
    map: ::MapDef,
    selected_tile: ::Pos,
    buttons: Vec<::draw::Button>,
}

impl MapEditor {
    pub fn new(_p: &::Platform, map: ::MapDef) -> MapEditor {
        MapEditor{
            camera: (0, 0),
            camera_speed: 5,
            pressed_keys: ::util::KeySet::new(),
            map: map,
            selected_tile: (0, 0),
            buttons: vec![
                ::draw::Button::new(_p, String::from("Add Column"), (10, 400), 1),
                ::draw::Button::new(_p, String::from("Add Row"), (120, 400), 2),
            ],
        }
    }
}

impl ::GameState for MapEditor {
    fn update(&mut self, _p: &::Platform) -> Option<Box<::GameState>> {
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

    fn render(&self, _p: &::Platform) {
        let white = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
        ::draw::map(_p, &self.map, self.camera);
        ::draw::tile_highlight(_p, self.camera, self.selected_tile);
        ::draw::text(_p, white, (10, 10), FontAlign::Left, "Welcome to the map editor!");
        self.buttons.iter().for_each(|button| button.render(_p));
    }

    fn handle_event(&mut self, _p: &::Platform, event: Event) -> Option<Box<::GameState>> {
        if let Some((keycode, is_down)) = self.pressed_keys.handle_key_event(event) {
            return if keycode == ::allegro::keycodes::KeyCode::Space && is_down {
                Some(Box::new(::states::game::Game::new(Some(self.map.clone()))))
            } else {
                None
            }
        }

        match event {
            ::allegro::Event::MouseButtonDown{ x, y, button, .. } if button == 1 => {
                let mut clicked_button = None;
                for button in self.buttons.iter() {
                    if button.clicked((x, y)) {
                        clicked_button = Some(button.get_id());
                        break;
                    }
                }
                if let Some(id) = clicked_button {
                    self.handle_button(id);
                    return None;
                }

                if let Some(pos) = ::draw::clicked_tile(&self.map, self.camera, (x, y)) {
                    self.selected_tile = pos;
                    return None;
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

impl MapEditor {
    fn handle_button(&mut self, id: u32) {
        match id {
            1 => self.add_column(),
            2 => self.add_row(),
            _ => (),
        }
    }

    fn add_column(&mut self) {
        let column = self.map.last().unwrap().clone();
        self.map.push(column);
    }

    fn add_row(&mut self) {
        for column in self.map.iter_mut() {
            let tile = column.last().unwrap().clone();
            column.push(tile);
        }
    }
}
