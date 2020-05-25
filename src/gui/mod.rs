//! # GUI
//!
//! `gui` contains the functions that handles the GUI

mod grid;
mod menu;
mod sound;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use fltk::{app::{App, AppScheme}, enums::Color, image::PngImage, prelude::{GroupExt, WidgetExt, WindowExt}, window::MenuWindow};
use crate::difficulty::Difficulty;

/// The GUI is represented here
pub struct Game {
    pub size: u8,
    pub difficulty: Difficulty,
    pub theme: AppScheme,
}

/// Returns a GUI
pub fn create() {
    let game = Game {
        size: 6,
        difficulty: Difficulty::Beginner,
        theme: AppScheme::Gtk,
    };
    let (app, mut window) = Game::init_gui(&game);
    let mut menu = menu::init(window.width());
    window.end();
    window.show();
    let game = Rc::new(RefCell::new(game));
    menu::add_entries(&mut menu, &game);
    app.run().unwrap();
}

/// Returns the newly created `App` and `Window`
///
/// # Arguments
///
/// * `game` - a game
fn init_gui(game: &Game) -> (App, MenuWindow) {
    let app = App::default();
    app.set_scheme(game.theme);
    let mut window = MenuWindow::new(100, 100, 600, 552, "YABinero").center_screen();
    if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
        window.set_icon(&icon);
    }
    window.set_color(Color::Light2);
    window.make_resizable(false);
    (app, window)
}

fn add_waiting_frame() {
}
