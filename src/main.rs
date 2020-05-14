//! # Yet Another Binero puzzle game

mod engine;
mod value;

use engine::Binero;

fn main() {
    let game = Binero::new(12);
    println!("New game: {}", game);
}
