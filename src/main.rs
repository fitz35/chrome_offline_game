
use brain::brain_train_pipeline;
use game::{Game};
use iced::{Settings, Application};
use program_args::ProgramArgs;
use structopt::StructOpt;


mod game;
mod entity;
mod utils;
mod params;
mod neurone;
mod brain;
mod program_args;


fn main() -> iced::Result {
    // get var env to say if we want to run the game or the brain train
    let args = ProgramArgs::from_args();

    // if we want to play the game
    if args.play {
        // run the game
        Game::run(Settings {
            antialiasing: true,
            ..Settings::default()
        })
    }else if args.brain_path.is_some() {
        // run the brain play
        // TODO : get the brain in the given path
        Game::run(Settings {
            antialiasing: true,
            ..Settings::default()
        })
    }else if args.folder_path.is_some() {
        // run the brain train
        brain_train_pipeline();
        return Ok(());
    }else {
        // error, we need to have at least one argument
        println!("You need to give at least one argument, run -h to see the help");
        return Ok(());
    }
}