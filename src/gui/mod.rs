//! # GUI
//!
//! `gui` contains the functions that handles the GUI

mod changing;
mod menu;
mod sound;
mod timer;
mod user_data;

use std::{cell::RefCell, path::Path, rc::Rc};
use fltk::{app::{App, AppScheme, screen_size}, dialog::{alert, message}, enums::Color, image::PngImage, menu::MenuBar, prelude::{GroupExt, WidgetExt, WindowExt}, window::MenuWindow};
use user_data::UserPrefs;
use changing::ChangingPart;

/// The GUI is represented here
pub struct Game {
    user_prefs: Rc<RefCell<UserPrefs>>,
    app: Rc<RefCell<App>>,
    window: MenuWindow,
    menu: MenuBar,
    changing: Rc<RefCell<ChangingPart>>,
}

impl Game {
    /// Returns the GUI of the game
    pub fn new() -> Game {
        let user_prefs = UserPrefs::new();
        let (app, window) = Game::init_gui(&user_prefs.theme());
        let menu = menu::init(window.width());
        let user_prefs = Rc::new(RefCell::new(user_prefs));
        let changing = Rc::new(RefCell::new(ChangingPart::new(menu.height(), window.width(), window.height())));
        Game {
            user_prefs,
            app,
            window,
            menu,
            changing,
        }
    }

    /// Shows the window of the game
    pub fn show_window(&mut self) {
        self.window.end();
        self.window.show();
    }

    /// Adds the menus to the menu bar
    pub fn add_menu_entries(&mut self) {
        menu::add_entries(&mut self.menu, &self.user_prefs, &self.changing, &self.app);
        menu::set_menu_items(&mut self.menu, &self.user_prefs);
    }

    /// Runs the game
    pub fn run_app(&self) {
        self.app.borrow().run().unwrap();
    }

    /// Returns the newly created `App` and `Window`
    ///
    /// # Arguments
    ///
    /// * `theme` - an `AppScheme`
    fn init_gui(theme: &AppScheme) -> (Rc<RefCell<App>>, MenuWindow) {
        let app = App::default();
        app.set_scheme(*theme);
        let mut window = MenuWindow::new(0, 0, 700, 552, "YABinero").center_screen();
        if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
            window.set_icon(Some(icon));
        }
        window.set_color(BG_COLOR);
        window.make_resizable(false);
        (Rc::new(RefCell::new(app)), window)
    }
}

/// Displays a popup with an error message
///
/// # Arguments
///
/// * `msg` - the error message
pub fn display_alert(msg: &str) {
    let (width, height) = screen_size();
    alert(width as i32 / 2 - 302, height as i32 / 2 - 14, msg);
}

/// Displays a popup with a message
///
/// # Arguments
///
/// * `msg` - the message
fn display_message(msg: &str) {
    let (width, height) = screen_size();
    message(width as i32 / 2 - 302, height as i32 / 2 - 14, msg);
}

pub const BG_COLOR: Color = Color::Light2;
pub const FG_COLOR: Color = Color::Black;
pub const SELECT_COLOR: Color = Color::Dark3;
pub const RO_FG_COLOR: Color = Color::Inactive;
pub const RO_SELECT_COLOR: Color = Color::Dark1;
