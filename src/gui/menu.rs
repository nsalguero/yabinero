use std::fmt;
use fltk::menu::MenuBar;
use tr::tr;

/// Returns an empty menu bar
pub fn init(width: i32) -> MenuBar {
    MenuBar::new(0, 0, width, 40, "")
}

pub fn add_entries(menu: &mut MenuBar) {
    
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

/// The `Game` submenu
enum GameSubmenu {
    New,
    BestScores,
    Quit,
}

impl fmt::Display for GameSubmenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printable = match *self {
            GameSubmenu::New => tr!("New"),
            GameSubmenu::BestScores => tr!("Best scores"),
            GameSubmenu::Quit => tr!("Quit"),
        };
        write!(f, "{}", printable)
    }
}

/// The `Options` submenu
enum OptionsSubmenu {
    Size,
    Difficulty,
    Sounds,
    Theme,
}

impl fmt::Display for OptionsSubmenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printable = match *self {
            OptionsSubmenu::Size => tr!("Game"),
            OptionsSubmenu::Difficulty => tr!("Options"),
            OptionsSubmenu::Sounds => tr!("Sounds"),
            OptionsSubmenu::Theme => tr!("Theme"),
        };
        write!(f, "{}", printable)
    }
}

/// The `Help` submenu
enum HelpSubmenu {
    About,
}

impl fmt::Display for OptionsSubmenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printable = match *self {
            HelpSubmenu::Size => tr!("About"),
        };
        write!(f, "{}", printable)
    }
}
