//! # Menu
//!
//! `menu` contains the functions that handles the menu

use std::{cell::RefCell, fmt, fs, path::Path, rc::Rc, sync::mpsc::Sender};
use fltk::{app::{App, AppScheme, quit}, button::Button, enums::Shortcut, group::ColorChooser, prelude::{MenuExt, WidgetBase, WidgetExt}, menu::{MenuBar, MenuFlag}};
use tr::tr;
use enum_iterator::all;
use regex::Regex;
use crate::enums::{Difficulty, Size};
use crate::gui::{BG_COLOR, SELECT_COLOR, BUTTON_HEIGHT, display_window, show, popup_window, user_data::{UserPrefs, BestScores}, changing::ChangingPart};

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
    add_undo(menu, changing);
    add_redo(menu, changing);
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
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::New, None), Shortcut::Ctrl | 'n', MenuFlag::Normal, Box::new(move |_: &mut MenuBar| {
        if let Some(t) = &tx {
            t.send(true).unwrap();
            ChangingPart::pause_game(&cloned_changing);
        }
        tx = Some(ChangingPart::new_game(&cloned_prefs, &cloned_changing));
    }));
}

/// Adds the "Game/Undo" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `changing` - the changing part of the GUI
fn add_undo(menu: &mut MenuBar, changing: &Rc<RefCell<ChangingPart>>) {
    let cloned_changing = Rc::clone(changing);
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::Undo, None), Shortcut::Ctrl | 'u', MenuFlag::Normal, Box::new(move |_: &mut MenuBar| {
        ChangingPart::undo(&cloned_changing);
    }));
}

/// Adds the "Game/Redo" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `changing` - the changing part of the GUI
fn add_redo(menu: &mut MenuBar, changing: &Rc<RefCell<ChangingPart>>) {
    let cloned_changing = Rc::clone(changing);
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::Redo, None), Shortcut::Ctrl | 'U', MenuFlag::Normal, Box::new(move |_: &mut MenuBar| {
        ChangingPart::redo(&cloned_changing);
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
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::BestScores, None), Shortcut::None, MenuFlag::Normal, Box::new(move |_: &mut MenuBar| {
        let best_scores = BestScores::new().best_scores(cloned_prefs.borrow().size(), cloned_prefs.borrow().difficulty());
        display_window(326, 230, &tr!("Best scores"), &best_scores, true, 184, None);
    }));
}

/// Adds the "Game/Quit" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
fn add_quit(menu: &mut MenuBar) {
    menu.add(&entry_label(&TopLevelMenu::Game, &Submenu::Quit, None), Shortcut::Ctrl | 'q', MenuFlag::Normal, Box::new(|_: &mut MenuBar| {
        quit();
    }));
}

/// Adds the "Options/Size/..." menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
/// * `user_prefs` - the user's preferences
fn add_sizes(menu: &mut MenuBar, user_prefs: &Rc<RefCell<UserPrefs>>) {
    for size in all::<Size>() {
        let cloned_prefs = Rc::clone(user_prefs);
        menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Size, Some(&format!("{}", size))), Shortcut::None, MenuFlag::Radio, Box::new(move |_: &mut MenuBar| {
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
    for difficulty in all::<Difficulty>() {
        let cloned_prefs = Rc::clone(user_prefs);
        menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Difficulty, Some(&format!("{}", difficulty))), Shortcut::None, MenuFlag::Radio, Box::new(move |_: &mut MenuBar| {
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
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Sounds, None), Shortcut::None, MenuFlag::Toggle, Box::new(move |_: &mut MenuBar| {
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
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Base))), Shortcut::None, MenuFlag::Radio, Box::new(move |_: &mut MenuBar| {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Base);
        cloned_app.borrow().with_scheme(AppScheme::Base);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gtk))), Shortcut::None, MenuFlag::Radio, Box::new(move |_: &mut MenuBar| {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Gtk);
        cloned_app.borrow().with_scheme(AppScheme::Gtk);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Gleam))), Shortcut::None, MenuFlag::Radio, Box::new(move |_: &mut MenuBar| {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Gleam);
        cloned_app.borrow().with_scheme(AppScheme::Gleam);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Plastic))), Shortcut::None, MenuFlag::Radio, Box::new(move |_: &mut MenuBar| {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Plastic);
        cloned_app.borrow().with_scheme(AppScheme::Plastic);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    let cloned_app = Rc::clone(app);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Theme, Some(&format!("{:?}", AppScheme::Oxy))), Shortcut::None, MenuFlag::Radio, Box::new(move |_: &mut MenuBar| {
        cloned_prefs.borrow_mut().set_theme(AppScheme::Oxy);
        cloned_app.borrow().with_scheme(AppScheme::Oxy);
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
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Colors, Some(&tr!("Color of actives boxes"))), Shortcut::None, MenuFlag::Normal, Box::new(move |_: &mut MenuBar| {
        display_color_chooser(&cloned_prefs, false);
    }));
    let cloned_prefs = Rc::clone(user_prefs);
    menu.add(&entry_label(&TopLevelMenu::Options, &Submenu::Colors, Some(&tr!("Color of inactives boxes"))), Shortcut::None, MenuFlag::Normal, Box::new(move |_: &mut MenuBar| {
        display_color_chooser(&cloned_prefs, true);
    }));
}

/// Creates a button
///
/// # Arguments
///
/// * `x` - the horizontal starting point
/// * `y` - the vertical starting point
/// * `title` - the title of the button
fn button(x: i32, y: i32, title: &str) -> Button {
    let mut button = Button::new(x, y, BUTTON_WIDTH, BUTTON_HEIGHT, "");
    button.set_label(title);
    button.set_color(BG_COLOR);
    button
}

/// Displays a color chooser
///
/// # Arguments
///
/// * `user_prefs` - the user's preferences
/// * `read_only` - whether or not the color is for read-only boxes
fn display_color_chooser(user_prefs: &Rc<RefCell<UserPrefs>>, read_only: bool) {
    let cloned_prefs = Rc::clone(user_prefs);
    let width = 200;
    let mut window = popup_window(width, 142, &tr!("Choose"));
    let chooser_height = 95;
    let chooser = ColorChooser::new(0, 0, width, chooser_height, "");
    let mut but_cancel = button(width / 2 - BUTTON_WIDTH - 5, chooser_height, &tr!("Cancel"));
    let mut but_ok = button(width / 2 + 5, chooser_height, &tr!("OK"));
    show(&mut window);
    let window = Rc::new(RefCell::new(window));
    let window2 = Rc::clone(&window);
    but_cancel.set_callback(Box::new(move |_: &mut Button| {
        window.borrow_mut().hide();
    }));
    but_ok.set_callback(Box::new(move |_: &mut Button| {
        let color = chooser.rgb_color();
        if read_only {
            cloned_prefs.borrow_mut().set_ro_color(color);
        } else {
            cloned_prefs.borrow_mut().set_color(color);
        }
        window2.borrow_mut().hide();
    }));
}

/// Returns the help of the game
fn about() -> String {
    let authors = authors();
    let authors_number = authors.len();
    let mut result = tr!("\t\tYet Another Binero puzzle game, version {}.", VERSION);
    result.push_str("\n\n\n");
    result.push_str(&tr!("This software is a mathematical puzzle game."));
    result.push_str(&new_line(authors_number));
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
    result.push_str(&new_line(authors_number));
    result.push_str(&tr!("This software is developped by:"));
    for author in authors {
        result.push_str(&format!("\n\t- {}.", author));
    }
    result.push_str(&new_line(authors_number));
    result.push_str(&tr!("This software is released under the following licence: {}.", LICENSE));
    result.push_str(&new_line(authors_number));
    result.push_str(&tr!("For more information, please see:"));
    result.push_str(&format!("\n{}.", HOMEPAGE));
    result
}

/// Returns the authors
fn authors() -> Vec<String> {
    let authors = env!("CARGO_PKG_AUTHORS");
    let re = Regex::new(r" <[^@]+@[^@]+>").unwrap();
    let authors = re.replace_all(authors, "").replace(":", "\n");
    authors.lines().map(String::from).collect()
}

/// Returns some new lines depending on the number of authors
///
/// # Arguments
///
/// * `authors_number` - the number of authors
fn new_line(authors_number: usize) -> String {
    if authors_number < 3 {
        String::from("\n\n\n")
    } else if authors_number < 6 {
        String::from("\n\n")
    } else {
        String::from("\n")
    }
}

/// Adds the "Help/About" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
fn add_about(menu: &mut MenuBar) {
    menu.add(&entry_label(&TopLevelMenu::Help, &Submenu::About, None), Shortcut::Ctrl | 'h', MenuFlag::Normal, Box::new(|_: &mut MenuBar| {
        let about = about();
        display_window(490, 510, &tr!("About"), &about, true, 460, None);
    }));
}

/// Adds the "Help/License" menu entry
///
/// # Arguments
///
/// * `menu` - a menu bar
fn add_license(menu: &mut MenuBar) {
    menu.add(&entry_label(&TopLevelMenu::Help, &Submenu::License, None), Shortcut::None, MenuFlag::Normal, Box::new(|_: &mut MenuBar| {
        let license = fs::read_to_string(Path::new("LICENSE")).unwrap();
        display_window(560, 600, &tr!("License"), &license, false, 11500, None);
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
    Undo,
    Redo,
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
            Submenu::Undo => tr!("Undo"),
            Submenu::Redo => tr!("Redo"),
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
const BUTTON_WIDTH: i32 = 70;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
const LICENSE: &'static str = env!("CARGO_PKG_LICENSE");
