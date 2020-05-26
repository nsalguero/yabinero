//! # User Data
//!
//! `user_data` contains the functions that handles the user's preferences and best scores

use crate::difficulty::Difficulty;
use fltk::app::AppScheme;

/// The user's preferences
pub struct UserPrefs {
    pub size: u8,
    pub difficulty: Difficulty,
    pub sounds: bool,
    pub theme: AppScheme,
}

impl UserPrefs {
    /// Returns the user's preferences
    pub fn new() -> UserPrefs {
        UserPrefs {
            size: 6, // FIXME use preferences dependency
            difficulty: Difficulty::Beginner, // FIXME use preferences dependency
            sounds: true, // FIXME use preferences dependency
            theme: AppScheme::Gtk, // FIXME use preferences dependency
        }
    }
}
