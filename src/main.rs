mod value;
mod engine;
mod gui;

use value::Value;
use engine::Grid;

fn main() {
    let first = Value::First;
    let second = Value::Second;
    let mut grid = Grid::new(5);
    grid.set(4, 2, first);
    grid.set(3, 2, second);
    println!("Grid: {}", grid);
}
