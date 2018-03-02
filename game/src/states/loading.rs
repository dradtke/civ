use allegro::{Color, MemoryBitmap};
use allegro_font::FontAlign;
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::u8;

#[no_mangle]
pub struct Loading {
    pub base_text: String,
    /// The number of dots to display.
    pub dot_count: i8,
    /// The number of dots after which they will be reset to 0.
    pub dot_max: i8,
    /// Number of frames before adding the next dot.
    pub dot_delay: i8,
    /// Incremented once each frame.
    pub dot_timer: i8,

    pub tilemap_recv: Option<Receiver<MemoryBitmap>>,
}

impl Clone for Loading {
    fn clone(&self) -> Loading {
        Loading{
            base_text: self.base_text.clone(),
            dot_count: self.dot_count,
            dot_max: self.dot_max,
            dot_delay: self.dot_delay,
            dot_timer: self.dot_timer,
            tilemap_recv: None,
        }
    }
}

impl Loading {
    pub fn new(_: &::Platform) -> Loading {
        Loading{
            base_text: String::from("Loading"),
            dot_count: 0,
            dot_max: 3,
            dot_delay: 15,
            dot_timer: 0,
            tilemap_recv: None,
        }
    }
}

impl ::GameState for Loading {
    fn init(&mut self, p: &::Platform) {
        let (tx, rx) = channel();
        self.tilemap_recv = Some(rx);
        let core = p.core.clone();
        thread::spawn(move || {
            let tilemap = ::assets::load_tilemap(&core);
            tx.send(tilemap).unwrap();
        });
    }

    fn render(&self, p: &::Platform) {
        let white = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
        let dots = (0..self.dot_count).map(|_| '.').collect::<String>();
        ::draw::text(p, white, (10, 10), FontAlign::Left, (String::from("Loading") + &dots).as_str());
    }

    fn update(&mut self, p: &::Platform) -> Option<Box<::GameState>> {
        if let Some(ref rx) = self.tilemap_recv {
            match rx.try_recv() {
                Ok(tilemap) => {
                    ::assets::init_tilemap(tilemap);
                    return Some(Box::new(::states::game::Game::new(None)));
                },
                // TODO: check what type of error this actually is
                Err(_) => {
                    // Handle the dot animation.
                    if self.dot_timer >= self.dot_delay {
                        self.dot_count = if self.dot_count < self.dot_max { self.dot_count + 1 } else { 0 };
                        self.dot_timer = 0;
                        if self.dot_count == 0 {
                            // Done loading, now let's actually load the bitmap.
                            return Some(Box::new(::states::game::Game::new(None)));
                        }
                    } else {
                        self.dot_timer += 1;
                    }

                    None
                },
            }
        } else {
            panic!("tilemap receiver not defined");
        }
    }

    fn box_clone(&self) -> Box<::GameState> {
        Box::new((*self).clone())
    }
}
