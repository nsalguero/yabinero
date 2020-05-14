//! # Engine
//!
//! `engine` contains the functions that create and solve a binero

mod grid;
mod history;

use std::fmt;
use rand::Rng;
use grid::Grid;
use history::{History, Item};
use crate::value::{self, Value};

/// A binero game is represented here
pub struct Binero {
    grid: Grid,
    history: History,
}

impl Binero {
    /// Returns a binero, with the given size, ready to be played
    ///
    /// # Arguments
    ///
    /// * `size` - an unsigned 8-bit integer that gives the size
    ///
    /// # Example
    ///
    /// ```
    /// use engine::Binero;
    /// let game = Binero::new(6);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `size` is an odd number
    pub fn new(size: u8) -> Binero {
        let mut result = Binero {
            grid: Grid::new(size),
            history: History::new(),
        };
        result.solve();
        result.history.clear();
        result
    }

    /// Try to solve a binero and returns if it could or not
    pub fn solve(&mut self) -> bool {
        let mut grid_can_be_solved = true;
        while !self.grid.is_full() {
            loop {
                let (some_value_put, backtrack_impossible) = self.put_mandatory_values();
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
            if !self.grid.is_full() {
                if !self.try_a_choice() {
                    grid_can_be_solved = self.backtrack_to_latest_choice();
                    if !grid_can_be_solved {
                        break;
                    }
                }
            }
        }
        grid_can_be_solved
    }

    /// Put a value in the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - an `Option<Value>`
    pub fn put(&mut self, x_axis: u8, y_axis: u8, value: Option<Value>) {
        let old_value = self.grid.put(x_axis, y_axis, value);
        self.history.push(x_axis, y_axis, old_value, value, true);
    }

    /// Cancels the latest action if it is possible
    pub fn undo(&mut self) -> Option<&Item> {
        if self.history.is_undo_possible() {
            Some(self.history.undo())
        } else {
            None
        }
    }

    /// Replays the next action that was undone if it is possible
    pub fn redo(&mut self) -> Option<&Item> {
        if self.history.is_redo_possible() {
            Some(self.history.redo())
        } else {
            None
        }
    }

    /// Put a mandatory value in the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - a `Value`
    fn put_a_mandatory_value(&mut self, x_axis: u8, y_axis: u8, value: Value) {
        let new_value = Some(value);
        let old_value = self.grid.put(x_axis, y_axis, new_value);
        self.history.push(x_axis, y_axis, old_value, new_value, false);
    }

    /// Try to put all mandatory values in the grid and returns whether or not at least one value was
    /// added and whether or not backtracking was impossible
    fn put_mandatory_values(&mut self) -> (bool, bool) {
        let mut some_value_put = false;
        for i in 0..self.grid.size() {
            for j in 0..self.grid.size() {
                if self.grid.get(i, j).is_none() {
                    let value = self.rand_value();
                    if self.grid.must_put(i, j, value) {
                        if self.grid.can_put(i, j, value) {
                            self.put_a_mandatory_value(i, j, value);
                        } else {
                            if !self.backtrack_to_latest_choice() {
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
    fn try_a_choice(&mut self) -> bool {
        for i in 0..self.grid.size() {
            for j in 0..self.grid.size() {
                if self.grid.get(i, j).is_none() {
                    let value = self.rand_value();
                    if self.grid.can_put(i, j, value) {
                        let new_value = Some(value);
                        let old_value = self.grid.put(i, j, new_value);
                        self.history.push(i, j, old_value, new_value, true);
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Try to backtrack to the latest choice in the history and returns whether or not it was possible
    fn backtrack_to_latest_choice(&mut self) -> bool {
        let latest_choice = self.history.latest_choice();
        match latest_choice {
            Some(choice) => {
                let current = self.history.current_item().unwrap();
                for i in choice..=current {
                    let item = self.history.undo();
                    self.grid.put(item.x_axis(), item.y_axis(), item.old_value());
                    if i == current {
                        if let Some(bad_value) = item.new_value() {
                            let (x_axis, y_axis) = (item.x_axis(), item.y_axis());
                            self.put_a_mandatory_value(x_axis, y_axis, value::the_other(bad_value));
                        }
                    }
                }
                true
            },
            None => false,
        }
    }

    /// Returns a random `Value`
    fn rand_value(&self) -> Value {
        let value = rand::thread_rng().gen_range(0, 2);
        value::from_u8(value).unwrap()
    }
}

impl fmt::Display for Binero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid: {}\nHistory: {}", self.grid, self.history)
    }
}
