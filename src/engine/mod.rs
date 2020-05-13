//! # Engine
//!
//! `engine` contains the functions that create and solve a binero

pub mod grid;

use rand::Rng;
use grid::Grid;
use crate::value;

/// Returns a binero grid, with the given size, ready to be played
///
/// # Arguments
///
/// * `size` - an unsigned 8-bit integer that gives the size
///
/// # Example
///
/// ```
/// use engine::create_grid;
/// let grid = create_grid(6);
/// ```
///
/// # Panics
///
/// Panics if `size` is an odd number
pub fn create_grid(size: u8) -> Grid {
    let mut result = Grid::new(size);
    for i in 0..result.size() {
        for j in 0..result.size() {
            let value = rand::thread_rng().gen_range(0, 2);
            if let Some(val) = value::from_u8(value) {
                if result.can_put(i, j, val) {
                    result.put(i, j, Some(val));
                } else {
                    result.put(i, j, Some(value::the_other(val)));
                }
            }
        }
    }
    result
}
