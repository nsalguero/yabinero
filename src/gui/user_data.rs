//! # User Data
//!
//! `user_data` contains the functions that handles the user's preferences and best scores

use crate::size::Size;
use crate::difficulty::Difficulty;
use fltk::app::AppScheme;

/// The user's preferences
pub struct UserPrefs {
    size: Size,
    difficulty: Difficulty,
    sounds: bool,
    theme: AppScheme,
}

impl UserPrefs {
    /// Returns the user's preferences
    pub fn new() -> UserPrefs {
        UserPrefs {
            size: Size::Side12, // TODO use preferences dependency
            difficulty: Difficulty::Beginner, // TODO use preferences dependency
            sounds: false, // TODO use preferences dependency
            theme: AppScheme::Gtk, // TODO use preferences dependency
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    pub fn sounds(&self) -> bool {
        self.sounds
    }

    pub fn set_sounds(&mut self, sounds: bool) {
        self.sounds = sounds;
    }

    pub fn theme(&self) -> AppScheme {
        self.theme
    }

    pub fn set_theme(&mut self, theme: AppScheme) {
        self.theme = theme;
    }
}
