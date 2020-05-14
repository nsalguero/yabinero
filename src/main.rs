//! # Yet Another Binero puzzle game

mod engine;
mod value;
mod difficulty;
mod gui;

use engine::Binero;
use difficulty::Difficulty;

fn main() {
    let mut game = Binero::new(12, Difficulty::Beginner);
    game.put(0, 0, None);
    game.put(0, 1, None);
    game.put(1, 1, None);
    game.put(2, 1, None);
    println!("{}", game.undo().unwrap());
    println!("{}", game.undo().unwrap());
    println!("{}", game.undo().unwrap());
    println!("{}", game.undo().unwrap());
    println!("{}", game.undo().is_none());
    println!("{}", game.redo().unwrap());
    println!("{}", game.redo().unwrap());
    println!("{}", game.redo().unwrap());
    println!("{}", game.redo().unwrap().x_axis());
    println!("{}", game.redo().is_none());
    println!("New game: {}", game);
}
