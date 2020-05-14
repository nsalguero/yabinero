//! # Yet Another Binero puzzle game

mod engine;
mod value;
mod gui;

use engine::Binero;

fn main() {
    let mut game = Binero::new(12);
    game.put(0, 0, None);
    game.put(0, 1, None);
    game.put(1, 1, None);
    game.put(2, 1, None);
    game.undo();
    game.undo();
    game.redo();
    println!("New game: {}", game);
}
