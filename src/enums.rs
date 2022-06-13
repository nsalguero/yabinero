//! # Enums
//!
//! `enums` contains several enums used in all the game

use std::fmt;
use tr::tr;
use enum_iterator::{all, Sequence};

/// The four possible levels of difficulty
#[derive(Debug, Clone, Copy, Sequence)]
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
        for a_difficulty in all::<Difficulty>() {
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

/// The possible sizes of a binero
#[derive(Copy, Clone, Debug, Sequence, PartialEq, Eq, Hash)]
pub enum Size {
    Side6,
    Side8,
    Side10,
    Side12,
    Side14,
    Side16,
}

impl Size {
    /// Returns the size as an unsigned 8-bit integer
    pub fn as_u8(&self) -> u8 {
        let size = format!("{:?}", self).replace("Side", "");
        size.parse().unwrap()
    }

    /// Returns the `Size` corresponding to a given string slice
    ///
    /// # Arguments
    ///
    /// * `size` - a string slice representing a size
    pub fn from_str(size: &str) -> Option<Size> {
        let side = format!("{}x{}", size, size);
        for a_size in all::<Size>() {
            if format!("{}", a_size) == side {
                return Some(a_size);
            }
        }
        None
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.as_u8();
        let printable = format!("{}x{}", size, size);
        write!(f, "{}", printable)
    }
}

/// The two possible values that can be put in a binero grid
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    First,
    Second,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Value::First => Value::FIRST_CHAR,
            Value::Second => Value::SECOND_CHAR,
        };
        write!(f, "{}", printable)
    }
}

impl Value {
    /// Returns a `Some(Value)` if the given character is correct and `None` otherwise
    ///
    /// # Arguments
    ///
    /// * `val` - a character
    pub fn from_char(val: char) -> Option<Value> {
        match val {
            Value::FIRST_CHAR => Some(Value::First),
            Value::SECOND_CHAR => Some(Value::Second),
            _ => None,
        }
    }

    /// Returns a `Some(Value)` if the given integer is correct and `None` otherwise
    ///
    /// # Arguments
    ///
    /// * `val` - an unsigned 8-bit integer
    pub fn from_u8(val: u8) -> Option<Value> {
        Value::from_char(char::from(val + 48))
    }

    /// Returns the other value
    pub fn the_other(&self) -> Value {
        match *self {
            Value::First => Value::Second,
            Value::Second => Value::First,
        }
    }

    const FIRST_CHAR: char = '0';
    const SECOND_CHAR: char = '1';
}
