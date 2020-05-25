//! # GUI
//!
//! `gui` contains the functions that handles the GUI

mod grid;
mod menu;
mod sound;

use fltk::{app::{App, AppScheme}, enums::Color, image::PngImage, prelude::{GroupExt, WidgetExt, WindowExt}, window::MenuWindow};
use std::path::Path;

/// The GUI is represented here
pub struct Game {
    app: App,
    window: MenuWindow,
}

impl Game {
    /// Returns a GUI
    pub fn new() -> Game {
        let (app, mut window) = Game::init_gui();
        let mut menu = menu::init(window.width());
        window.end();
        window.show();
        menu::add_entries(&mut menu);
        app.run().unwrap();
        Game {
            app,
            window,
        }
    }

    /// Returns the newly created `App` and `Window`
    fn init_gui() -> (App, MenuWindow) {
        let app = App::default();
        app.set_scheme(AppScheme::Gtk);
        let mut window = MenuWindow::new(100, 100, 600, 552, "YABinero").center_screen();
        if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
            window.set_icon(&icon);
        }
        window.set_color(Color::Light2);
        window.make_resizable(false);
        (app, window)
    }
}
