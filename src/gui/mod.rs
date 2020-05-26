//! # GUI
//!
//! `gui` contains the functions that handles the GUI

mod grid;
mod menu;
mod sound;
mod user_data;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use fltk::{app::{App, AppScheme}, enums::Color, image::PngImage, menu::MenuBar, prelude::{GroupExt, WidgetExt, WindowExt}, window::MenuWindow};
use user_data::UserPrefs;

/// The GUI is represented here
pub struct Game {
    user_prefs: Rc<RefCell<UserPrefs>>,
    app: App,
    window: MenuWindow,
    menu: MenuBar,
}

impl Game {
    /// Returns the GUI of the game
    pub fn new() -> Game {
        let user_prefs = UserPrefs::new();
        let (app, window) = Game::init_gui(&user_prefs.theme);
        let menu = menu::init(window.width());
        let user_prefs = Rc::new(RefCell::new(user_prefs));
        Game {
            user_prefs,
            app,
            window,
            menu,
        }
    }

    /// Shows the window of the game
    pub fn show_window(&mut self) {
        self.window.end();
        self.window.show();
    }

    /// Adds the menus to the menu bar
    pub fn add_menu_entries(&mut self) {
        menu::add_entries(&mut self.menu, &self.user_prefs);
        menu::set_menu_items(&mut self.menu, &self.user_prefs);
    }

    /// Runs the game
    pub fn run_app(&self) {
        self.app.run().unwrap();
    }

    /// Returns the newly created `App` and `Window`
    ///
    /// # Arguments
    ///
    /// * `theme` - an `AppScheme`
    fn init_gui(theme: &AppScheme) -> (App, MenuWindow) {
        let app = App::default();
        app.set_scheme(*theme);
        let mut window = MenuWindow::new(0, 0, 600, 552, "YABinero").center_screen();
        if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
            window.set_icon(&icon);
        }
        window.set_color(Color::Light2);
        window.make_resizable(false);
        (app, window)
    }
}
