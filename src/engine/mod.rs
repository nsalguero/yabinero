//! # Engine
//!
//! `engine` contains the functions that create and solve a binero

pub mod grid;
pub mod history;

use rand::Rng;
use grid::Grid;
use history::History;
use crate::value::{self, Value};

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
pub fn create_grid(size: u8) -> (Grid, History) {
    let mut grid = Grid::new(size);
    let mut history = History::new();
    solve(&mut grid, &mut history);
    (grid, History::new())
}

pub fn solve(grid: &mut Grid, history: &mut History) {
    let grid_size = grid.size();

    while !grid.is_full() {
        loop {
            let mut put_mandatory_values = || -> bool {
                let mut result = false;
                for i in 0..grid_size {
                    for j in 0..grid_size {
                        let value = rand_value();
                        if grid.must_put(i, j, value) {
                            let new_value = Some(value);
                            let old_value = grid.put(i, j, new_value);
                            history.push(i, j, old_value, new_value, false);
                            result = true;
                        }
                    }
                }
                result
            };

            if put_mandatory_values() {
                break;
            }
        }

        let mut try_a_choice = || {
            for i in 0..grid_size {
                for j in 0..grid_size {
                    let mut value = rand_value();
                    if grid.can_put(i, j, value) {
                        let new_value = Some(value);
                        let old_value = grid.put(i, j, new_value);
                        history.push(i, j, old_value, new_value, true);
                    }
                }
            }
        };

        try_a_choice();
    }
}

fn rand_value() -> Value {
    let value = rand::thread_rng().gen_range(0, 2);
    value::from_u8(value).unwrap()
}
