//! # Value
//!
//! `value` describes the values that can be put in the grid of the game

use std::fmt;

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
