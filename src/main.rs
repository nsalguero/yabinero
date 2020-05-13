//! # Yet Another Binero puzzle game

mod value;
mod engine;

fn main() {
    let (grid, history) = engine::create_grid(12);
    println!("New grid: {}", grid);
}
