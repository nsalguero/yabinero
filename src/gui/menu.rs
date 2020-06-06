//! # Menu
//!
//! `menu` contains the functions that handles the menu

use std::process::exit;
use std::fmt;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::Sender;
use fltk::{app::AppScheme, enums::{Align, Color, Shortcut}, frame::Frame, prelude::{GroupExt, MenuExt, WidgetExt, WindowExt}, menu::{MenuBar, MenuFlag}, group::Scroll, window::Window};
use tr::tr;
use crate::size::Size;
use crate::difficulty::Difficulty;
use crate::gui::user_data::UserPrefs;
use crate::gui::changing::ChangingPart;
use enum_iterator::IntoEnumIterator;

/// Returns an empty menu bar
///
/// # Arguments
///
/// * `width` - the width of the menu bar
pub fn init(width: i32) -> MenuBar {
    let mut menu = MenuBar::new(0, 0, width, MENU_HEIGHT, "");
    menu.set_color(Color::Light2);
    menu.set_selection_color(Color::Dark3);
    menu
}

/// Adds the entries to the menu bar
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
/// * `changing` - the changing part of the GUI
pub fn add_entries(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>, changing: &Rc<RefCell<ChangingPart>>) {
    add_game_entries(menu, user_prefs, changing);
    add_options_entries(menu, user_prefs);
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
fn add_options_entries(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    add_sizes(menu, user_prefs);
    add_difficulties(menu, user_prefs);
    add_sounds(menu, user_prefs);
    add_themes(menu, user_prefs);
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
        // TODO display a window with the best scores for the current size and the current difficulty
        println!("{} {}", cloned_prefs.borrow().size(), cloned_prefs.borrow().difficulty());
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
fn add_themes(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Base))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Base);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gtk))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Gtk);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gleam))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Gleam);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Plastic))), Shortcut::None, MenuFlag::Radio, Box::new(move || {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Plastic);
    }));
}

fn about() -> String {
    let mut result = tr!("\t\tYet Another Binero puzzle game, version 1.0.0.");
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
        let mut window = Window::new(0, 0, 490, 460, &tr!("About")).center_screen();
        window.make_modal(true);
        window.make_resizable(false);
        let about = about();
        let mut frame = Frame::new(0, 0, 20, 460, &about);
        frame.set_align(Align::AlignRight);
        let mut scroll = Scroll::new(0, 0, 490, 460, "");
        scroll.add(&frame);
        scroll.set_color(Color::Light2);
        window.end();
        window.show();
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
        scroll.set_color(Color::Light2);
        window.end();
        window.show();
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
            Submenu::About => tr!("About"),
            Submenu::License => tr!("License"),
        };
        write!(f, "{}", printable)
    }
}

const MENU_HEIGHT: i32 = 40;
