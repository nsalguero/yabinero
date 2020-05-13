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
            Value::First => FIRST_CHAR,
            Value::Second => SECOND_CHAR,
        };
        write!(f, "{}", printable)
    }
}

/// Returns a `Some(Value)` if the given character is correct and `None` otherwise
///
/// # Arguments
///
/// * `val` - a character
///
/// # Example
///
/// ```
/// if let Some(val) = value::from_char('0') {
///     println!("{}", val);
/// }
/// ```
pub fn from_char(val: char) -> Option<Value> {
    match val {
        FIRST_CHAR => Some(Value::First),
        SECOND_CHAR => Some(Value::Second),
        _ => None,
    }
}

/// Returns a `Some(Value)` if the given integer is correct and `None` otherwise
///
/// # Arguments
///
/// * `val` - an unsigned 8-bit integer
///
/// # Example
///
/// ```
/// if let Some(val) = value::from_u8(0) {
///     println!("{}", val);
/// }
/// ```
pub fn from_u8(val: u8) -> Option<Value> {
    from_char(char::from(val + 48))
}

/// Returns the other `Value` of a given `Value`
///
/// # Arguments
///
/// * `value` - a `Value`
///
/// # Example
///
/// ```
/// let other_val = value::the_other(Value::First);
/// ```
pub fn the_other(value: Value) -> Value {
    match value {
        Value::First => Value::Second,
        Value::Second => Value::First,
    }
}

const FIRST_CHAR: char = '0';
const SECOND_CHAR: char = '1';
