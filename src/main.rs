//! # Yet Another Binero puzzle game

mod value;
mod engine;
mod gui;

use value::{Value, value_from_char};
use engine::grid::Grid;
use engine::create_grid;

fn main() {
    //let first = Value::First;
    //let second = Value::Second;
    //let mut grid = Grid::new(6);
    //grid.put(4, 2, first);
    //grid.put(3, 2, second);
    //grid.put(1, 2, second);
    //grid.put(5, 2, second);
    //println!("Grid: {}", grid);
    //if let Some(val) = value_from_char('0') {
    //    println!("{}", val);
    //}
    //if let Some(val) = grid.put(3, 2, first) {
    //    println!("{}", val);
    //}
    //println!("{}", grid.can_put(2, 2, second));
    let grid = create_grid(6);
    println!("New grid: {}", grid);
}
