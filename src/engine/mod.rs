//! # Engine
//!
//! `engine` contains the functions that create and solve a binero

pub mod grid;
pub mod history;

use rand::Rng;
use grid::Grid;
use history::{History, Item};
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
/// use engine;
/// let grid = engine::create_grid(6);
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

/// Try to solve a binero grid and returns if it could or not
///
/// # Arguments
///
/// * `grid` - a grid
/// * `history` - the history of the game
///
/// # Example
///
/// ```
/// use engine;
/// let success = engine::solve(grid, history);
/// ```
pub fn solve(grid: &mut Grid, history: &mut History) -> bool {
    let mut grid_can_be_solved = true;
    while !grid.is_full() {
        loop {
            let (some_value_put, backtrack_impossible) = put_mandatory_values(grid, history);
            if backtrack_impossible {
                grid_can_be_solved = false;
                break;
            }
            if !some_value_put {
                break;
            }
        }
        if !grid_can_be_solved {
            break;
        }
        if !grid.is_full() {
            if !try_a_choice(grid, history) {
                grid_can_be_solved = backtrack_to_latest_choice(grid, history);
                break;
            }
        }
    }
    grid_can_be_solved
}

/// Put a mandatory value in the grid
///
/// # Arguments
///
/// * `grid` - a grid
/// * `history` - the history of the game
/// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
/// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
/// * `value` - a `Value`
fn put_a_mandatory_value(grid: &mut Grid, history: &mut History, x_axis: u8, y_axis: u8, value: Value) {
    let new_value = Some(value);
    let old_value = grid.put(x_axis, y_axis, new_value);
    history.push(x_axis, y_axis, old_value, new_value, false);
}

/// Try to put all mandatory values in the grid and returns whether or not at least one value was
/// added and whether or not backtracking was impossible
///
/// # Arguments
///
/// * `grid` - a grid
/// * `history` - the history of the game
fn put_mandatory_values(grid: &mut Grid, history: &mut History) -> (bool, bool) {
    let mut some_value_put = false;
    for i in 0..grid.size() {
        for j in 0..grid.size() {
            if grid.get(i, j).is_none() {
                let value = rand_value();
                if grid.must_put(i, j, value) {
                    if grid.can_put(i, j, value) {
                        put_a_mandatory_value(grid, history, i, j, value);
                    } else {
                        if !backtrack_to_latest_choice(grid, history) {
                            return (false, true);
                        }
                    }
                    some_value_put = true;
                }
            }
        }
    }
    (some_value_put, false)
}

/// Try to put a choice in the grid and returns whether or not it was possible
///
/// # Arguments
///
/// * `grid` - a grid
/// * `history` - the history of the game
fn try_a_choice(grid: &mut Grid, history: &mut History) -> bool {
    for i in 0..grid.size() {
        for j in 0..grid.size() {
            if grid.get(i, j).is_none() {
                let mut value = rand_value();
                if grid.can_put(i, j, value) {
                    let new_value = Some(value);
                    let old_value = grid.put(i, j, new_value);
                    history.push(i, j, old_value, new_value, true);
                    return true;
                }
            }
        }
    }
    false
}

/// Try to backtrack to the latest choice in the history and returns whether or not it was possible
///
/// # Arguments
///
/// * `grid` - a grid
/// * `history` - the history of the game
fn backtrack_to_latest_choice(grid: &mut Grid, history: &mut History) -> bool {
    let latest_choice = history.latest_choice();
    match latest_choice {
        Some(choice) => {
            let latest = history.current_item().unwrap() + 1;
            for i in choice..latest {
                let item = history.undo();
                grid.put(item.x_axis(), item.y_axis(), item.old_value());
                if i == latest - 1 {
                    if let Some(bad_value) = item.new_value() {
                        let (x_axis, y_axis) = (item.x_axis(), item.y_axis());
                        put_a_mandatory_value(grid, history, x_axis, y_axis, value::the_other(bad_value));
                    }
                }
            }
            true
        },
        None => false,
    }
}

/// Returns a random `Value`
fn rand_value() -> Value {
    let value = rand::thread_rng().gen_range(0, 2);
    value::from_u8(value).unwrap()
}
