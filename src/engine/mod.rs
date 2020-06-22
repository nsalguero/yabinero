//! # Engine
//!
//! `engine` contains the functions that create and solve a binero

mod grid;
pub mod history;

use std::fmt;
use rand::{Rng, prelude::*};
use grid::Grid;
use history::{History, Item};
use crate::enums::{Difficulty, Size, Value};

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
    /// * `size` - a size
    /// * `difficulty` - a level of difficulty
    pub fn new(size: Size, difficulty: Difficulty) -> Binero {
        let mut result = Binero {
            grid: Grid::new(size),
            history: History::new(),
        };
        result.try_to_solve();
        result.history.clear();
        result.make_playable(difficulty);
        result
    }

    /// Try to solve a binero and returns if it could or not
    pub fn try_to_solve(&mut self) -> bool {
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

    /// Try to put a value in the grid and returns whether or not it was possible
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - an `Option<Value>`
    pub fn try_to_put(&mut self, x_axis: u8, y_axis: u8, value: Option<Value>) -> bool {
        match value {
            Some(val) => {
                if self.grid.can_put(x_axis, y_axis, val) {
                    if self.grid.must_put(x_axis, y_axis, val) {
                        self.put_a_mandatory_value(x_axis, y_axis, val);
                    } else {
                        self.put_a_choice(x_axis, y_axis, value);
                    }
                } else {
                    return false;
                }
            },
            None => self.put_a_choice(x_axis, y_axis, value),
        }
        true
    }

    /// Cancels the latest action and returns it if it is possible or returns `None`
    pub fn try_to_undo(&mut self) -> Option<&Item> {
        if self.history.is_undo_possible() {
            let item = self.history.undo();
            self.grid.put(item.x_axis(), item.y_axis(), item.old_value());
            Some(item)
        } else {
            None
        }
    }

    /// Replays the next action that was undone and returns it if it is possible or returns `None`
    pub fn try_to_redo(&mut self) -> Option<&Item> {
        if self.history.is_redo_possible() {
            let item = self.history.redo();
            self.grid.put(item.x_axis(), item.y_axis(), item.new_value());
            Some(item)
        } else {
            None
        }
    }

    /// Clears the history of the game
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Returns wheter or not the grid is full
    pub fn is_full(&self) -> bool {
        self.grid.is_full()
    }

    /// Returns a value from the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    pub fn get(&self, x_axis: u8, y_axis: u8) -> Option<Value> {
        self.grid.get(x_axis, y_axis)
    }

    /// Returns the size of the grid
    pub fn size(&self) -> Size {
        self.grid.size()
    }

    /// Put a choice in the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - an `Option<Value>`
    fn put_a_choice(&mut self, x_axis: u8, y_axis: u8, value: Option<Value>) {
        let old_value = self.grid.put(x_axis, y_axis, value);
        self.history.push(x_axis, y_axis, old_value, value, true);
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

    /// Returns the mandatory value for a place in the grid if it exists or `None` if there is no
    /// mandatory value for that place
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    fn mandatory_value(&mut self, x_axis: u8, y_axis: u8) -> Option<Value> {
        let value = self.rand_value();
        if self.grid.must_put(x_axis, y_axis, value) {
            Some(value)
        } else {
            let other_value = value.the_other();
            if self.grid.must_put(x_axis, y_axis, other_value) {
                Some(other_value)
            } else {
                None
            }
        }
    }

    /// Try to put all mandatory values in the grid and returns whether or not at least one value was
    /// added and whether or not backtracking was impossible
    fn put_mandatory_values(&mut self) -> (bool, bool) {
        let mut some_value_put = false;
        for i in 0..self.grid.size().as_u8() {
            for j in 0..self.grid.size().as_u8() {
                if self.grid.get(i, j).is_none() {
                    if let Some(value) = self.mandatory_value(i, j) {
                        if self.grid.can_put(i, j, value) {
                            self.put_a_mandatory_value(i, j, value);
                        } else {
                            let result = self.backtrack_to_latest_choice();
                            return (result, !result);
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
        for i in 0..self.grid.size().as_u8() {
            for j in 0..self.grid.size().as_u8() {
                if self.grid.get(i, j).is_none() {
                    let value = self.rand_value();
                    if self.grid.can_put(i, j, value) {
                        self.put_a_choice(i, j, Some(value));
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
                            let other_value = bad_value.the_other();
                            if self.grid.can_put(x_axis, y_axis, other_value) {
                                self.put_a_mandatory_value(x_axis, y_axis, other_value);
                            } else {
                                return self.backtrack_to_latest_choice();
                            }
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
        Value::from_u8(value).unwrap()
    }

    /// Removes a certain amount of values from the grid according to the given difficulty
    ///
    /// # Arguments
    ///
    /// * `difficulty` - a level of difficulty
    fn make_playable(&mut self, difficulty: Difficulty) {
        let total = (self.grid.size().as_u8() as u16).pow(2);
        let max_removed: u16 = match difficulty {
            Difficulty::Beginner => total / 2,
            Difficulty::Easy => total * 2 / 3,
            Difficulty::Medium => total * 3 / 4,
            Difficulty::Hard => total,
        };
        let indexes = self.shuffle_indexes();
        let mut first = true;
        for &(i, j) in indexes.iter() {
            if self.grid.empty_values() >= max_removed {
                break;
            }
            let value = self.grid.put(i, j, None);
            if first {
                first = false;
                continue;
            }
            self.check_a_value(i, j, value);
        }
    }

    /// Checks whether or not a value can be removed from the grid keeping a unique solution for
    /// the binero
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - an `Option<Value>`
    fn check_a_value(&mut self, x_axis: u8, y_axis: u8, value: Option<Value>) {
        let other_value = value.unwrap().the_other();
        self.grid.put(x_axis, y_axis, Some(other_value));
        let non_unique_solution = self.try_to_solve();
        while self.try_to_undo().is_some() {
        }
        self.grid.put(x_axis, y_axis, None);
        self.history.clear();
        if non_unique_solution {
            self.grid.put(x_axis, y_axis, value);
        }
    }

    /// Shuffles the x and y axis and returns them
    fn shuffle_indexes(&self) -> Vec<(u8, u8)> {
        let mut rng = rand::thread_rng();
        let mut result: Vec<(u8, u8)> = Vec::new();
        for i in 0..self.grid.size().as_u8() {
            for j in 0..self.grid.size().as_u8() {
                result.push((i, j));
            }
        }
        result.shuffle(&mut rng);
        result
    }
}

impl fmt::Display for Binero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid: {}\nHistory: {}", self.grid, self.history)
    }
}
