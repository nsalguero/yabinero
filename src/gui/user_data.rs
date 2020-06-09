//! # User Data
//!
//! `user_data` contains the functions that handles the user's preferences and best scores

std::collections::HashMap
use std::str::FromStr;
use preferences::{AppInfo, PreferencesMap, Preferences};
use crate::size::Size;
use crate::difficulty::Difficulty;
use fltk::{app::{AppScheme, screen_size}, dialog::alert};
use tr::tr;

/// The user's preferences
pub struct UserPrefs {
    faves: PreferencesMap<String>,
}

impl UserPrefs {
    /// Returns the user's preferences
    pub fn new() -> UserPrefs {
        let load_result = PreferencesMap::load(&APP_INFO, PREFS_KEY);
        if load_result.is_ok() {
            UserPrefs {
                faves: load_result.unwrap(),
            }
        } else {
            let mut faves = PreferencesMap::new();
            faves.insert("size".to_owned(), "6".to_owned());
            faves.insert("difficulty".to_owned(), "Beginner".to_owned());
            faves.insert("sounds".to_owned(), "true".to_owned());
            faves.insert("theme".to_owned(), "Gtk".to_owned());
            let result = UserPrefs {
                faves,
            };
            result.save();
            result
        }
    }

    /// Returns the current size
    pub fn size(&self) -> Size {
        if let Some(size) = self.faves.get("size") {
            if let Some(result) = Size::from_str(size) {
                result
            } else {
                UserPrefs::bad_size()
            }
        } else {
            UserPrefs::bad_size()
        }
    }

    /// Sets the size
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    pub fn set_size(&mut self, size: Size) {
        self.faves.insert("size".to_owned(), format!("{}", size.as_u8()));
        self.save();
    }

    /// Returns the current difficulty
    pub fn difficulty(&self) -> Difficulty {
        if let Some(difficulty) = self.faves.get("difficulty") {
            if let Some(result) = Difficulty::from_str(difficulty) {
                result
            } else {
                UserPrefs::bad_difficulty()
            }
        } else {
            UserPrefs::bad_difficulty()
        }
    }

    /// Sets the difficulty
    ///
    /// # Arguments
    ///
    /// * `difficulty` - a difficulty
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.faves.insert("difficulty".to_owned(), format!("{:?}", difficulty));
        self.save();
    }

    /// Returns whether or not the sounds must be played
    pub fn sounds(&self) -> bool {
        if let Some(sounds) = self.faves.get("sounds") {
            if let Ok(result) = bool::from_str(sounds) {
                result
            } else {
                UserPrefs::bad_sounds()
            }
        } else {
            UserPrefs::bad_sounds()
        }
    }

    /// Sets whether or not the sounds must be played
    ///
    /// # Arguments
    ///
    /// * `sounds` - whether or not the sounds must be played
    pub fn set_sounds(&mut self, sounds: bool) {
        self.faves.insert("sounds".to_owned(), format!("{}", sounds));
        self.save();
    }

    /// Returns the current theme
    pub fn theme(&self) -> AppScheme {
        if let Some(theme) = self.faves.get("theme") {
            match theme.as_str() {
                "Base" => AppScheme::Base,
                "Gtk" => AppScheme::Gtk,
                "Gleam" => AppScheme::Gleam,
                "Plastic" => AppScheme::Plastic,
                _ => {
                    UserPrefs::bad_theme()
                },
            }
        } else {
            UserPrefs::bad_theme()
        }
    }

    /// Sets the theme
    ///
    /// # Arguments
    ///
    /// * `theme` - a theme
    pub fn set_theme(&mut self, theme: AppScheme) {
        self.faves.insert("theme".to_owned(), format!("{:?}", theme));
        self.save();
    }

    /// Displays an error
    ///
    /// # Arguments
    ///
    /// * `msg` - an error message
    fn display_error(msg: &str) {
        let (width, height) = screen_size();
        alert(width as i32 / 2 - 302, height as i32 / 2 - 14, msg);
    }

    /// Saves the user's preferences
    fn save(&self) {
        let save_result = self.faves.save(&APP_INFO, PREFS_KEY);
        if !save_result.is_ok() {
            UserPrefs::display_error(&tr!("User preferences cannot be saved!"));
        }
    }

    /// Returns the default size when the size cannot be read from the user's preferences
    fn bad_size() -> Size {
        UserPrefs::display_error(&tr!("Bad size!"));
        Size::Side6
    }

    /// Returns the default difficulty when the difficulty cannot be read from the user's preferences
    fn bad_difficulty() -> Difficulty {
        UserPrefs::display_error(&tr!("Bad difficulty!"));
        Difficulty::Beginner
    }

    /// Returns `true` when the choice about playing the sounds cannot be read from the user's preferences
    fn bad_sounds() -> bool {
        UserPrefs::display_error(&tr!("Unable to know whether or not the sounds must be played!"));
        true
    }

    /// Returns the default theme when the theme cannot be read from the user's preferences
    fn bad_theme() -> AppScheme {
        UserPrefs::display_error(&tr!("Bad theme!"));
        AppScheme::Base
    }
}

const APP_INFO: AppInfo = AppInfo{name: "yabinero", author: "Nicolas Salguero"};
const PREFS_KEY: &'static str = "yabinero";
