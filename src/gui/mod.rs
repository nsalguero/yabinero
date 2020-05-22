//! # GUI
//!
//! `gui` contains the functions that handles the GUI

use fltk::{app::{App, AppScheme}, button::*, frame::Frame, image::PngImage, input::Input, menu::*, window::MenuWindow};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use tr::tr;

mod grid;
mod menu;
mod sound;

use crate::engine::Binero;

/// The GUI is represented here
pub struct Game {
    binero: Option<Binero>,
    app: App,
    wind: Window,
}

impl Game {
    /// Returns a GUI
    pub fn new() -> Game {
        let (app, wind) = Game::init_gui();
        wind.end();
        wind.show();
        app.run().unwrap();
        Game {
            binero: None,
            app,
            wind,
        }
    }

    /// Returns the newly created `App` and `Window`
    fn init_gui() -> (App, Window) {
        let app = App::default();
        let mut wind = MenuWindow::new(100, 100, 400, 520, "Hello from rust").center_screen();
        if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
            wind.set_icon(&icon);
        }
        wind.set_color(Color::Light2);
        wind.make_resizable(false);
        (app, wind)
    }
}
