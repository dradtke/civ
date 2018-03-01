use std::collections::HashSet;

/// KeySet is a wrapper around HashSet<u32> that automatically handles
/// keycode conversions, along with convenience methods for keeping track
/// of the keyboard's state.
#[derive(Clone)]
pub struct KeySet(HashSet<u32>);

impl KeySet {
    pub fn new() -> KeySet {
        KeySet(HashSet::new())
    }

    pub fn mark_down(&mut self, keycode: ::allegro::keycodes::KeyCode) {
        self.0.insert(keycode as u32);
    }

    pub fn mark_up(&mut self, keycode: ::allegro::keycodes::KeyCode) {
        self.0.remove(&(keycode as u32));
    }

    pub fn is_down(&self, keycode: ::allegro::keycodes::KeyCode) -> bool {
        self.0.contains(&(keycode as u32))
    }

    pub fn is_up(&self, keycode: ::allegro::keycodes::KeyCode) -> bool {
        !self.is_down(keycode)
    }

    /// An Allegro event can be passed into here to update the set's state.
    /// It returns true if the event was a key-down or key-up event, otherwise
    /// false.
    pub fn handle_key_event(&mut self, event: ::allegro::Event) -> bool {
        match event {
            ::allegro::Event::KeyDown{ keycode, .. } => { self.0.insert(keycode as u32); true },
            ::allegro::Event::KeyUp{ keycode, .. } => { self.0.remove(&(keycode as u32)); true },
            _ => false,
        }
    }
}
