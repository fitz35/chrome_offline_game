use std::env;

use brain::brain_train_pipeline;
use game::Game;
use iced::{Settings, Application};


mod game;
mod entity;
mod utils;
mod params;
mod neurone;
mod brain;


fn main() -> iced::Result {
    // get var env to say if we want to run the game or the brain train
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "train" {
        // run the brain train
    
        brain_train_pipeline();




        return Ok(());
    }


    // need Application to be run
    Game::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}