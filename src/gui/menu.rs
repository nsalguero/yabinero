//! # Menu
//!
//! `menu` contains the functions that handles the menu

use std::{cell::RefCell, fmt, fs, path::Path, process::exit, rc::Rc, sync::mpsc::Sender};
use fltk::{app::{App, AppScheme}, button::Button, enums::{Align, Shortcut}, frame::Frame, group::ColorChooser, prelude::{GroupExt, MenuExt, WidgetExt, WindowExt}, menu::{MenuBar, MenuFlag}, group::Scroll, window::Window};
use tr::tr;
use enum_iterator::IntoEnumIterator;
use crate::enums::{Difficulty, Size};
use crate::gui::{BG_COLOR, SELECT_COLOR, user_data::{UserPrefs, BestScores}, changing::ChangingPart};

/// Returns an empty menu bar
///
/// # Arguments
///
/// * `width` - the width of the menu bar
pub fn init(width: i32) -> MenuBar {
    let mut menu = MenuBar::new(0, 0, width, MENU_HEIGHT, "");
    menu.set_color(BG_COLOR);
    menu.set_selection_color(SELECT_COLOR);
    menu
}

/// Adds the entries to the menu bar
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
/// * `changing` - the changing part of the GUI
/// * `app` - the app
pub fn add_entries(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>, changing: &Rc<RefCell<ChangingPart>>, app: &Rc<RefCell<App>>) {
    add_game_entries(menu, user_prefs, changing);
    add_options_entries(menu, user_prefs, app);
    add_help_entries(menu);
}

/// Sets the menu items according to the user's preferences
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
pub fn set_menu_items(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    let size = format!("{}", user_prefs.borrow().size());
    let size = entry_label(&TopLevelMenu::Options, &Submenu::Size, Some(&size));
    if let Some(mut menu_item) = menu.find_item(&size) {
        menu_item.set();
    }
    let difficulty = format!("{}", user_prefs.borrow().difficulty());
    let difficulty = entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&difficulty));
    if let Some(mut menu_item) = menu.find_item(&difficulty) {
        menu_item.set();
    }
    if user_prefs.borrow().sounds() {
        if let Some(mut menu_item) = menu.find_item(&entry_label(&TopLevelMenu::Options, &Submenu::Sounds, None)) {
            menu_item.set();
        }
    }
    let theme = format!("{:?}", user_prefs.borrow().theme());
    let theme = entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&theme));
    if let Some(mut menu_item) = menu.find_item(&theme) {
        menu_item.set();
    }
}

/// Adds the entries to the "Game" menu
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
/// * `changing` - the changing part of the GUI
fn add_game_entries(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>, changing: &Rc<RefCell<ChangingPart>>) {
    add_new_game(menu, user_prefs, changing);
    add_best_scores(menu, user_prefs);
    add_quit(menu);
}

/// Adds the entries to the "Options" menu
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
/// * `app` - the app
fn add_options_entries(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>, app: &Rc<RefCell<App>>) {
    add_sizes(menu, user_prefs);
    add_difficulties(menu, user_prefs);
    add_sounds(menu, user_prefs);
    add_themes(menu, user_prefs, app);
    add_colors(menu, user_prefs);
}

/// Adds the entries to the "Help" menu
///
/// # Arguments
///
/// * `menu` - a menu bar
fn add_help_entries(menu: &mut MenuBar) {
    add_about(menu);
    add_license(menu);
}

/// Adds the "Game/New" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
/// * `changing` - the changing part of the GUI
fn add_new_game(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>, changing: &Rc<RefCell<ChangingPart>>) {
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_changing = Rc::clone(changing);
    let mut tx: Option<Sender<bool>> = None;
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::New, None), Shortcut::Ctrl + 'n', MenuFlag::Normal, Box::new(move || {
        if let Some(t) = &tx {
            t.send(true).unwrap();
            ChangingPart::pause_game(&cloned_changing);
        }
        tx = Some(ChangingPart::new_game(&cloned_prefs, &cloned_changing));
    }));
}

/// Adds the "Game/Best scores" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
fn add_best_scores(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::BestScores, None), Shortcut::None, MenuFlag::Normal, Box::new(move || {
        let mut window = Window::new(0, 0, 326, 230, &tr!("Best scores")).center_screen();
        window.make_modal(true);
        window.make_resizable(false);
        let best_scores = BestScores::new().best_scores(cloned_prefs.borrow().size(), cloned_prefs.borrow().difficulty());
        let mut frame = Frame::new(0, 0, 20, 184, &best_scores);
        frame.set_align(Align::Right);
        let mut scroll = Scroll::new(0, 0, 326, 230, "");
        scroll.add(&frame);
        scroll.set_color(BG_COLOR);
        let mut but_ok = Button::new(128, 184, 70, 40, &tr!("OK"));
        but_ok.set_color(BG_COLOR);
        window.end();
        window.show();
        but_ok.set_callback(Box::new(move || {
            window.hide();
        }));
    }));
}

/// Adds the "Game/Quit" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
fn add_quit(menu: &mut MenuBar) {
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::Quit, None), Shortcut::Ctrl + 'q', MenuFlag::Normal, Box::new(|| {
        exit(0);
    }));
}

/// Adds the "Options/Size/..." menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
fn add_sizes(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    for size in Size::into_enum_iter() {
        let cloned_prefs = Rc::clone(user_prefs);
        menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some(&format!("{}", size))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
            cloned_prefs.borrow_mut().set_size(size);
        }));
    }
}

/// Adds the "Options/Difficulty/..." menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
fn add_difficulties(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    for difficulty in Difficulty::into_enum_iter() {
        let cloned_prefs = Rc::clone(user_prefs);
        menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&format!("{}", difficulty))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
            cloned_prefs.borrow_mut().set_difficulty(difficulty);
        }));
    }
}

/// Adds the "Options/Sounds" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
fn add_sounds(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Sounds, None), Shortcut::None, MenuFlag::Toggle, Box::new(move || {
        let old_value = cloned_prefs.borrow().sounds();
        cloned_prefs.borrow_mut().set_sounds(!old_value);
    }));
}

/// Adds the "Options/Theme/..." menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
/// * `app` - the app
fn add_themes(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>, app: &Rc<RefCell<App>>) {
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Base))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Base);
        cloned_app.borrow().set_scheme(AppScheme::Base);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gtk))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Gtk);
        cloned_app.borrow().set_scheme(AppScheme::Gtk);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gleam))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Gleam);
        cloned_app.borrow().set_scheme(AppScheme::Gleam);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Plastic))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Plastic);
        cloned_app.borrow().set_scheme(AppScheme::Plastic);
    }));
}

/// Adds the "Options/Colors/..." menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
fn add_colors(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Colors, Some(&tr!("Color of actives boxes"))), Shortcut::None, MenuFlag::Normal, Box::new(move || {
        display_color_chooser(&cloned_prefs, false);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Colors, Some(&tr!("Color of inactives boxes"))), Shortcut::None, MenuFlag::Normal, Box::new(move || {
        display_color_chooser(&cloned_prefs, true);
    }));
}

/// Displays a color chooser
///
/// # Arguments
///
/// * `user_prefs` - the user's preferences
/// * `read_only` - whether or not the color is for read-only boxes
fn display_color_chooser(user_prefs: &Rc<RefCell<UserPrefs>>, read_only: bool) {
    let cloned_prefs = Rc::clone(user_prefs);
    let mut window = Window::new(0, 0, 200, 142, &tr!("Choose")).center_screen();
    window.make_modal(true);
    window.make_resizable(false);
    let chooser = ColorChooser::new(0, 0, 200, 95, "");
    let mut but_ok = Button::new(65, 100, 70, 40, &tr!("OK"));
    but_ok.set_color(BG_COLOR);
    window.end();
    window.show();
    but_ok.set_callback(Box::new(move || {
        let color = chooser.rgb_color();
        if read_only {
            cloned_prefs.borrow_mut().set_ro_color(color);
        } else {
            cloned_prefs.borrow_mut().set_color(color);
        }
        window.hide();
    }));
}

/// Returns the help of the game
fn about() -> String {
    let mut result = tr!("\t\tYet Another Binero puzzle game, version 1.2.0.");
    result.push_str("\n\n\n");
    result.push_str(&tr!("This software is a mathematical puzzle game."));
    result.push_str("\n\n\n");
    result.push_str(&tr!("The aim of the game is to fill in a grid with 0 and 1 respecting"));
    result.push_str("\n");
    result.push_str(&tr!("two constraints:"));
    result.push_str("\n\n\t- ");
    result.push_str(&tr!("In each line or column, there must be the same number"));
    result.push_str("\n\t  ");
    result.push_str(&tr!("of the two values."));
    result.push_str("\n\n\t- ");
    result.push_str(&tr!("In each line or column, the same value cannot be side by"));
    result.push_str("\n\t  ");
    result.push_str(&tr!("side more than twice."));
    result.push_str("\n\n\n");
    result.push_str(&tr!("Developper: Nicolas Salguero."));
    result.push_str("\n\n\n");
    result.push_str(&tr!("This software is released under the GNU General Public Licence"));
    result.push_str("\n");
    result.push_str(&tr!("version 3.0 or any later version."));
    result.push_str("\n\n\n");
    result.push_str(&tr!("For more information, please see:"));
    result.push_str("\nhttps://github.com/nsalguero/yabinero");
    result
}

/// Adds the "Help/About" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
fn add_about(menu: &mut MenuBar) {
    menu.add(&entry_label(&TopLevelMenu::Help, &Submenu::About, None), Shortcut::Ctrl + 'h', MenuFlag::Normal, Box::new(|| {
        let mut window = Window::new(0, 0, 490, 510, &tr!("About")).center_screen();
        window.make_modal(true);
        window.make_resizable(false);
        let about = about();
        let mut frame = Frame::new(0, 0, 20, 460, &about);
        frame.set_align(Align::Right);
        let mut scroll = Scroll::new(0, 0, 490, 510, "");
        scroll.add(&frame);
        scroll.set_color(BG_COLOR);
        let mut but_ok = Button::new(210, 460, 70, 40, &tr!("OK"));
        but_ok.set_color(BG_COLOR);
        window.end();
        window.show();
        but_ok.set_callback(Box::new(move || {
            window.hide();
        }));
    }));
}

/// Adds the "Help/License" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
fn add_license(menu: &mut MenuBar) {
    menu.add(&entry_label(&TopLevelMenu::Help, &Submenu::License, None), Shortcut::None, MenuFlag::Normal, Box::new(|| {
        let mut window = Window::new(0, 0, 560, 600, &tr!("License")).center_screen();
        window.make_modal(true);
        window.make_resizable(false);
        let license = fs::read_to_string(Path::new("LICENSE")).unwrap();
        let frame = Frame::new(0, 0, 540, 11500, &license);
        let mut scroll = Scroll::new(0, 0, 560, 600, "");
        scroll.add(&frame);
        scroll.set_color(BG_COLOR);
        let mut but_ok = Button::new(245, 11500, 70, 40, &tr!("OK"));
        but_ok.set_color(BG_COLOR);
        window.end();
        window.show();
        but_ok.set_callback(Box::new(move || {
            window.hide();
        }));
    }));
}

/// Returns the label of an entry of the menu
///
/// # Arguments
///
/// * `top_level` - a top level entry of the menu
/// * `submenu` - a submenu entry
/// * `subsubmenu` - an optional entry under the submenu
fn entry_label(top_level: &TopLevelMenu, submenu: &Submenu, subsubmenu: Option<&str>) -> String {
    match subsubmenu {
        Some(subsub) => format!("{}/{}/{}\t", top_level, submenu, subsub),
        None => format!("{}/{}\t", top_level, submenu),
    }
}

/// The top level menus
enum TopLevelMenu {
    Game,
    Options,
    Help,
}

impl fmt::Display for TopLevelMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TopLevelMenu::Game => tr!("Game"),
            TopLevelMenu::Options => tr!("Options"),
            TopLevelMenu::Help => tr!("Help"),
        };
        write!(f, "{}", printable)
    }
}

/// The submenus
enum Submenu {
    New,
    BestScores,
    Quit,
    Size,
    Difficulty,
    Sounds,
    Theme,
    Colors,
    About,
    License,
}

impl fmt::Display for Submenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Submenu::New => tr!("New"),
            Submenu::BestScores => tr!("Best scores"),
            Submenu::Quit => tr!("Quit"),
            Submenu::Size => tr!("Size"),
            Submenu::Difficulty => tr!("Difficulty"),
            Submenu::Sounds => tr!("Sounds"),
            Submenu::Theme => tr!("Theme"),
            Submenu::Colors => tr!("Colors"),
            Submenu::About => tr!("About"),
            Submenu::License => tr!("License"),
        };
        write!(f, "{}", printable)
    }
}

const MENU_HEIGHT: i32 = 40;
