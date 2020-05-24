use std::process::exit;
use std::fmt;
use fltk::menu::MenuBar;
use tr::tr;
use crate::difficulty::Difficulty;

/// Returns an empty menu bar
pub fn init(width: i32) -> MenuBar {
    let mut menu = MenuBar::new(0, 0, width, 40, "");
    menu.set_color(Color::Light2);
    menu.set_selection_color(Color::Dark3);
}

pub fn add_entries(menu: &mut MenuBar) {
    menu.add(entry_label(TopLevelMenu::Game, Submenu::New, None), Shortcut::Ctrl + 'n', MenuFlag::MenuDivider, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Game, Submenu::BestScores, None), Shortcut::None, MenuFlag::MenuDivider, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Game, Submenu::Quit, None), Shortcut::Ctrl + 'q', MenuFlag::Normal, Box::new(|| {
        exit(0);
    }));
    menu.add(entry_label(TopLevelMenu::Options, Submenu::Size), Shortcut::None, MenuFlag::Submenu, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Options, Submenu::Difficulty, format!("{}", Difficulty::Beginner)), Shortcut::None, MenuFlag::Normal, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Options, Submenu::Difficulty, format!("{}", Difficulty::Easy)), Shortcut::None, MenuFlag::Normal, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Options, Submenu::Difficulty, format!("{}", Difficulty::Medium)), Shortcut::None, MenuFlag::Normal, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Options, Submenu::Difficulty, format!("{}", Difficulty::Hard)), Shortcut::None, MenuFlag::Normal, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Options, Submenu::Sounds, None), Shortcut::None, MenuFlag::Toggle, Box::new(|| {
    }));
    menu.add(entry_label(TopLevelMenu::Options, Submenu::Theme), Shortcut::None, MenuFlag::Submenu, Box::new(|| {
    }));
}

fn entry_label(topLevel: &TopLevelMenu, submenu: &Submenu, subsubmenu: Option<&str>) -> String {
    match subsubmenu {
        Some(subsub) => format!("{}/{}/{}\t", topLevel, submenu, subsub),
        None => format!("{}/{}\t", topLevel, submenu),
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
        let mut printable = match *self {
            GameSubmenu::New => tr!("New"),
            GameSubmenu::BestScores => tr!("Best scores"),
            GameSubmenu::Quit => tr!("Quit"),
            OptionsSubmenu::Size => tr!("Size"),
            OptionsSubmenu::Difficulty => tr!("Difficulty"),
            OptionsSubmenu::Sounds => tr!("Sounds"),
            OptionsSubmenu::Theme => tr!("Theme"),
            HelpSubmenu::About => tr!("About"),
            HelpSubmenu::License => tr!("License"),
        };
        write!(f, "{}", printable)
    }
}
