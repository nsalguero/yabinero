//! # GUI
//!
//! `gui` contains the functions that handles the GUI

mod changing;
mod menu;
mod sound;
mod timer;
mod user_data;

use std::{cell::RefCell, path::Path, rc::Rc};
use fltk::{app::{App, AppScheme}, button::ReturnButton, enums::{Align, Color}, group::Scroll, image::{PngImage, SvgImage}, frame::Frame, menu::MenuBar, prelude::{GroupExt, ImageExt, WidgetExt, WindowExt}, window::{MenuWindow, Window}};
use tr::tr;
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
        show(&mut self.window);
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
        app.with_scheme(*theme);
        let mut window = MenuWindow::new(0, 0, 700, 552, "YABinero");
        window = init_window(window, false);
        if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
            window.set_icon(Some(icon));
        }
        (Rc::new(RefCell::new(app)), window)
    }
}

/// Creates a modal window
///
/// # Arguments
///
/// * `width` - the width of the window
/// * `height` - the height of the window
/// * `title` - the title of the window
fn popup_window(width: i32, height: i32, title: &str) -> Window {
    let window = Window::new(0, 0, width, height, title);
    init_window(window, true)
}

/// Sets some parameters to a window
///
/// # Arguments
///
/// * `window` - a window
/// * `modal` - whether or not the window is a modal one
fn init_window<T: WindowExt>(mut window: T, modal: bool) -> T {
    window.set_color(BG_COLOR);
    window.make_resizable(false);
    window.make_modal(modal);
    window.center_screen()
}

/// Creates a `ReturnButton`
///
/// # Arguments
///
/// * `window_width` - the width of the window
/// * `y` - the vertical starting point
fn return_button(window_width: i32, y: i32) -> ReturnButton {
    let mut button = ReturnButton::new((window_width - RET_BUTTON_WIDTH) / 2, y, RET_BUTTON_WIDTH, BUTTON_HEIGHT, &tr!("Close"));
    button.set_color(BG_COLOR);
    button
}

/// Displays a modal window
///
/// # Arguments
///
/// * `width` - the width of the window
/// * `height` - the height of the window
/// * `title` - the title of the window
/// * `content` - the displayed content
/// * `right_align` - whether or not the content of the internal frame is right aligned
/// * `frame_height` - the height of the internal frame
/// * `icon` - the name of an icon
fn display_window(width: i32, height: i32, title: &str, content: &str, right_align: bool, frame_height: i32, icon: Option<&str>) {
    let mut window = popup_window(width, height, title);
    let frame_width = if right_align {
        20
    } else {
        width - 20
    };
    let mut frame = Frame::new(0, 0, frame_width, frame_height, content);
    if right_align {
        frame.set_align(Align::Right);
    }
    if let Some(ic) = icon {
        if let Ok(mut img) = SvgImage::load(&Path::new("icons").join(ic.to_owned() + ".svg")) {
            img.scale(80, 80, true, true);
            frame.set_image(Some(img));
        }
    }
    let mut scroll = Scroll::new(0, 0, width, height, "");
    scroll.add(&frame);
    scroll.set_color(BG_COLOR);
    let mut button = return_button(width, frame_height);
    show(&mut window);
    button.set_callback(Box::new(move || {
        window.hide();
    }));
}

/// Displays a popup with an error message
///
/// # Arguments
///
/// * `msg` - the error message
fn display_alert(msg: &str) {
    display_window(500, 170, "", msg, false, 120, Some("ko"));
}

/// Displays a popup with a message
///
/// # Arguments
///
/// * `msg` - the message
fn display_message(msg: &str) {
    display_window(500, 170, "", msg, false, 120, Some("ok"));
}

/// Shows a window
///
/// # Arguments
///
/// * `window` - a window
fn show(window: &mut impl WindowExt) {
    window.end();
    window.show();
}

const BG_COLOR: Color = Color::Light2;
const FG_COLOR: Color = Color::Black;
const SELECT_COLOR: Color = Color::Dark3;
const RO_FG_COLOR: Color = Color::Inactive;
const RO_SELECT_COLOR: Color = Color::Dark1;

const BUTTON_HEIGHT: i32 = 40;
const RET_BUTTON_WIDTH: i32 = 100;
