//! # Difficulty
//!
//! `difficulty` describes the levels of difficulty

use std::fmt;
use tr::tr;
use enum_iterator::IntoEnumIterator;

/// The four possible levels of difficulty
#[derive(Debug, Clone, Copy, IntoEnumIterator)]
pub enum Difficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// Returns the `Difficulty` corresponding to a given string slice
    ///
    /// # Arguments
    ///
    /// * `difficulty` - a string slice representing a difficulty
    pub fn from_str(difficulty: &str) -> Option<Difficulty> {
        for a_difficulty in Difficulty::into_enum_iter() {
            if format!("{:?}", a_difficulty) == difficulty {
                return Some(a_difficulty);
            }
        }
        None
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Difficulty::Beginner => tr!("Beginner"),
            Difficulty::Easy => tr!("Easy"),
            Difficulty::Medium => tr!("Medium"),
            Difficulty::Hard => tr!("Hard"),
        };
        write!(f, "{}", printable)
    }
}
