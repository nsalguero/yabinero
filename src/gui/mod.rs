//! # GUI
//!
//! `gui` contains the functions that handles the GUI

use fltk::{app::{App, AppScheme}, button::*, frame::Frame, image::PngImage, input::Input, menu::*, window::MenuWindow};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use tr::{tr, tr_init};

mod grid;
mod menu;
mod sound;

use crate::engine::Binero;

/// The GUI is represented here
pub struct Game {
    binero: Option<Binero>,
    app: App,
}

impl Game {
    /// Returns a GUI
    pub fn new() -> Game {
        let app = Game::init_gui();
        app.run().unwrap();
        Game {
            binero: None,
            app,
        }
    }

    /// Returns the newly created `App`
    fn init_gui() -> App {
        let app = App::default();
        app
    }
}
