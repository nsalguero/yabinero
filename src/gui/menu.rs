//! # Menu
//!
//! `menu` contains the functions that handles the menu

use std::process::exit;
use std::fmt;
use fltk::{app::AppScheme, enums::{Color, Shortcut}, prelude::{MenuExt, WidgetExt}, menu::{MenuBar, MenuFlag}};
use tr::tr;
use crate::difficulty::Difficulty;

/// Returns an empty menu bar
///
/// # Arguments
///
/// * `width` - the width of the menu
pub fn init(width: i32) -> MenuBar {
    let mut menu = MenuBar::new(0, 0, width, MENU_HEIGHT, "");
    menu.set_color(Color::Light2);
    menu.set_selection_color(Color::Dark3);
    menu
}

/// Add the entries to the menu
///
/// # Arguments
///
/// * `menu` - the menu
pub fn add_entries(menu: &mut MenuBar) {
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::New, None), Shortcut::Ctrl + 'n', MenuFlag::MenuDivider, Box::new(|| {
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
        let mut printable = match *self {
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
