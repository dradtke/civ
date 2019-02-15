use std::collections::HashSet;
use std::time::Instant;

/// KeySet is a wrapper around HashSet<u32> that automatically handles
/// keycode conversions, along with convenience methods for keeping track
/// of the keyboard's state.
#[derive(Clone)]
pub struct KeySet(HashSet<u32>);

#[allow(dead_code)]
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

    pub fn handle_key_event(&mut self, event: ::allegro::Event) -> Option<(::allegro::keycodes::KeyCode, bool)> {
        match event {
            ::allegro::Event::KeyDown{ keycode, .. } => { self.mark_down(keycode); Some((keycode, true)) },
            ::allegro::Event::KeyUp{ keycode, .. } => { self.mark_up(keycode); Some((keycode, false)) },
            _ => None,
        }
    }
}

/// Utility method for measuring the time it takes to execute a block of code. Example:
///
///     ::util::measure_elapsed("method name", || {
///         ...
///     });
///
#[allow(dead_code)]
pub fn measure_elapsed<F: FnOnce()>(name: &'static str, f: F) {
    let start = Instant::now();
    f();
    let elapsed = Instant::now().duration_since(start);
    let elapsed_millis = (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() as u64)/1000000;
    println!("{} elapsed time: {} milliseconds", name, elapsed_millis);
}
