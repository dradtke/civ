#[macro_use] extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;
extern crate allegro_primitives;
extern crate libloading;
extern crate game;

use allegro_font::FontAddon;
use allegro_image::ImageAddon;
use allegro_primitives::PrimitivesAddon;
use libloading::Library;
use game::{GameState, Platform};
use std::fs;
use std::sync::Arc;

const FPS:           f64 = 60.0;
const SCREEN_WIDTH:  i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

struct Application;

impl Application {
    fn new() -> Application {
        Application{}
        // Application(Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error)))
    }

    fn init(&self, p: &Platform, state: &mut Box<GameState>) {
        let lib = Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error));
        unsafe {
            let f = lib.get::<fn(&Platform, &mut Box<GameState>)>(b"init\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `init`: {}", error));
            f(p, state);
        }
    }

    fn update(&self, p: &Platform, state: &mut Box<GameState>) -> Option<Box<GameState>> {
        let lib = Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error));
        unsafe {
            let f = lib.get::<fn(&Platform, &mut Box<GameState>) -> Option<Box<GameState>>>(b"update\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `update`: {}", error));
            f(p, state)
        }
    }

    fn render(&self, p: &Platform, state: &Box<GameState>) {
        let lib = Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error));
        unsafe {
            let f = lib.get::<fn(&Platform, &Box<GameState>)>(b"render\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `render`: {}", error));
            f(p, state);
        }
    }

    fn handle_event(&self, p: &Platform, state: &mut Box<GameState>, event: allegro::Event) -> Option<Box<GameState>> {
        let lib = Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error));
        unsafe {
            let f = lib.get::<fn(&Platform, &mut Box<GameState>, allegro::Event) -> Option<Box<GameState>>>(b"handle_event\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `handle_event`: {}", error));
            f(p, state, event)
        }
    }

    fn clean_up(&self, state: Box<GameState>) {
        let lib = Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error));
        unsafe {
            let f = lib.get::<fn(Box<GameState>)>(b"clean_up\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `clean_up`: {}", error));
            f(state)
        }
    }
}

const LIB_PATH: &'static str = "../game/target/debug/libgame.dylib";

allegro_main!
{
    let core = allegro::Core::init().unwrap();
    let platform = Platform{
        font_addon: FontAddon::init(&core).unwrap_or_else(|msg| panic!(msg)),
        image_addon: ImageAddon::init(&core).unwrap_or_else(|msg| panic!(msg)),
        primitives_addon: PrimitivesAddon::init(&core).unwrap_or_else(|msg| panic!(msg)),
        display: allegro::Display::new(&core, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap(),
        core: Arc::new(core),
    };

    platform.display.set_window_title("Hello, Allegro!");

    platform.core.install_keyboard().unwrap();
    platform.core.install_mouse().unwrap();

    let app = Application::new();
    let mut last_modified = fs::metadata(LIB_PATH).unwrap().modified().unwrap();
    let mut state = game::new_state(&platform);
    app.init(&platform, &mut state);

    let timer = allegro::Timer::new(&platform.core, 1.0 / FPS).unwrap();
    let q = allegro::EventQueue::new(&platform.core).unwrap();
    q.register_event_source(platform.display.get_event_source());
    platform.core.get_keyboard_event_source().map(|src| q.register_event_source(src));
    platform.core.get_mouse_event_source().map(|src| q.register_event_source(src));
    q.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();

    'main: loop {
        if redraw && q.is_empty() {
            app.render(&platform, &state);
            platform.core.flip_display();
            redraw = false;
        }

        match q.wait_for_event() {
            allegro::DisplayClose{..} => break 'main,
            allegro::TimerTick{..} => {
                if let Ok(Ok(modified)) = fs::metadata(LIB_PATH).map(|m| m.modified()) {
                    if modified > last_modified {
                        /*
                        drop(app);
                        app = Application::new();
                        */
                        last_modified = modified;
                    }
                }
                if let Some(new_state) = app.update(&platform, &mut state) {
                    state = new_state;
                    app.init(&platform, &mut state);
                }
                redraw = true;
            },
            e => {
                if let Some(new_state) = app.handle_event(&platform, &mut state, e) {
                    state = new_state;
                    app.init(&platform, &mut state);
                }
            },
        }
    }

    println!("Cleaning up...");
    app.clean_up(state);
    //mem::forget(state);
    println!("Bye!");
}
