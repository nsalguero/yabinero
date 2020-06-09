//! # Size
//!
//! `size` describes the possible sizes of a binero

use std::fmt;
use enum_iterator::IntoEnumIterator;

/// The possible sizes of a binero
#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq, Eq, Hash)]
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
        for a_size in Size::into_enum_iter() {
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
