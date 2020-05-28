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
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.as_u8();
        let printable = format!("{}x{}", size, size);
        write!(f, "{}", printable)
    }
}
