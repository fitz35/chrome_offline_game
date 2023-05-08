use std::time::Instant;

use rand::Rng;

mod game;
mod entity;
mod utils;

fn main() {
    let mut game = game::Game::new(Instant::now(), "test");
    let random_number1: u32 = game.rng.gen();
    let random_number2: u32 = game.rng.gen();
    let random_number3: u32 = game.rng.gen();
    let random_number4: u32 = game.rng.gen();
    println!("test rng : {}", random_number1);
    println!("test rng : {}", random_number2);
    println!("test rng : {}", random_number3);
    println!("test rng : {}", random_number4);
}
