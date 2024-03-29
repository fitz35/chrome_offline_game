
use std::{fs::File, io::BufReader};

use brain::{brain_train_pipeline, IntermediateResult};
use game::{Game, CustomFlags};
use iced::{Settings, Application, window};
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
        let params;
        if args.params_path.is_some() {
            params = params::GameParameters::new_from_file(args.params_path.unwrap().as_str());
        }else{
            params = params::GameParameters::new_default();
        }
        // run the game
        Game::run(Settings {
            antialiasing: true,
            flags : CustomFlags::Play(params.clone()),
            window: window::Settings {
                position: window::Position::Centered,
                size: (params.game_width as u32, params.game_height as u32),
                ..window::Settings::default()
            },
            ..Settings::default()
        })
    }else if args.brain_path.is_some() {
        // run the brain play
        let params;
        if args.params_path.is_some() {
            params = params::GameParameters::new_from_file(args.params_path.unwrap().as_str());
        }else{
            params = params::GameParameters::new_default();
        }


        let file = File::open(args.brain_path.unwrap()).expect("Unable to open file");
        let reader = BufReader::new(file);

        let inter : IntermediateResult = serde_json::from_reader(reader).expect("Unable to read file");
        if inter.brains.len() == 0 {
            println!("The brain file is empty");
            return Ok(());
        }
        Game::run(Settings {
            antialiasing: true,
            flags : CustomFlags::Brain(inter.brains[0].clone(), params.clone()),
            window: window::Settings {
                position: window::Position::Centered,
                size: (params.game_width as u32, params.game_height as u32),
                ..window::Settings::default()
            },
            ..Settings::default()
        })
    }else if args.folder_path.is_some() {
        // run the brain train
        brain_train_pipeline(args.folder_path);
        return Ok(());
    }else {
        // error, we need to have at least one argument
        println!("You need to give at least one argument, run -h to see the help");
        return Ok(());
    }
}