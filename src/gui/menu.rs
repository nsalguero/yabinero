//! # Menu
//!
//! `menu` contains the functions that handles the menu

use std::process::exit;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use fltk::{app::AppScheme, enums::{Color, Shortcut}, prelude::{MenuExt, WidgetExt}, menu::{MenuBar, MenuFlag}};
use tr::tr;
use crate::engine::Binero;
use crate::difficulty::Difficulty;
use crate::gui::user_data::UserPrefs;

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
pub fn add_entries(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::New, None), Shortcut::Ctrl + 'n', MenuFlag::MenuDivider, Box::new(move || {
        let binero = Binero::new(cloned_prefs.borrow().size, cloned_prefs.borrow().difficulty);
    }));
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::BestScores, None), Shortcut::None, MenuFlag::MenuDivider, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::Quit, None), Shortcut::Ctrl + 'q', MenuFlag::Normal, Box::new(|| {
        exit(0);
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some("6x6")), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some("8x8")), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some("10x10")), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some("12x12")), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some("14x14")), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some("16x16")), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&format!("{}", Difficulty::Beginner))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&format!("{}", Difficulty::Easy))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&format!("{}", Difficulty::Medium))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&format!("{}", Difficulty::Hard))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Sounds, None), Shortcut::None, MenuFlag::Toggle, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Base))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gtk))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gleam))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Plastic))), Shortcut::None, MenuFlag::Radio, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Help, &Submenu::About, None), Shortcut::Ctrl + 'h', MenuFlag::Normal, Box::new(|| {
    }));
    menu.add(&entry_label(&TopLevelMenu::Help, &Submenu::License, None), Shortcut::None, MenuFlag::Normal, Box::new(|| {
    }));
}

/// Sets the menu items according to the user's preferences
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
pub fn set_menu_items(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    let size = user_prefs.borrow().size;
    let size = format!("{}x{}", size, size);
    let size = entry_label(&TopLevelMenu::Options, &Submenu::Size, Some(&size));
    if let Some(mut menu_item) = menu.find_item(&size) {
        menu_item.set();
    }
    let difficulty = format!("{}", user_prefs.borrow().difficulty);
    let difficulty = entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&difficulty));
    if let Some(mut menu_item) = menu.find_item(&difficulty) {
        menu_item.set();
    }
    if user_prefs.borrow().sounds {
        if let Some(mut menu_item) = menu.find_item(&entry_label(&TopLevelMenu::Options, &Submenu::Sounds, None)) {
            menu_item.set();
        }
    }
    let theme = format!("{:?}", user_prefs.borrow().theme);
    let theme = entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&theme));
    if let Some(mut menu_item) = menu.find_item(&theme) {
        menu_item.set();
    }
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
