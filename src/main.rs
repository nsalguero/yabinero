//! # Yet Another Binero puzzle game

mod engine;
mod value;
mod difficulty;
mod gui;

use engine::Binero;
use difficulty::Difficulty;

fn main() {
    let mut game = Binero::new(12, Difficulty::Beginner);
    println!("New game: {}", game);
    println!("Can be solved: {}", game.try_to_solve());
    println!("New game: {}", game);
}
