use allegro::Event;

#[no_mangle]
#[derive(Clone)]
pub struct Game {
    map: ::MapDef,
    x_start: f32,
    y_start: f32,
}

impl Game {
    pub fn new(map: Option<::MapDef>) -> Game {
        let map = map.unwrap_or(vec![
            vec![0, 0, 0, 1, 0, 2, 2, 0, 0],
            vec![0, 1, 1, 1, 5, 2, 2, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 2, 0, 0],
        ]);
        Game{
            x_start: 50.0,
            y_start: 50.0,
            map: map,
        }
    }
}

impl ::GameState for Game {
    fn update(&mut self, _: &::Platform) -> Option<Box<::GameState>> {
        None
    }

    fn render(&self, p: &::Platform) {
        ::draw_map(p, &self.map, self.x_start, self.y_start);
    }

    fn handle_event(&mut self, p: &::Platform, event: Event) -> Option<Box<::GameState>> {
        match event {
            ::allegro::Event::KeyDown{ keycode, ..} if keycode == ::allegro::keycodes::KeyCode::Space => {
                Some(Box::new(::states::map_editor::MapEditor::new(self.map.clone())))
            },
            _ => None
        }
    }

    fn box_clone(&self) -> Box<::GameState> {
        Box::new((*self).clone())
    }
}
