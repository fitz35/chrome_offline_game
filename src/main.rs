use game::Game;
use iced::{Settings, Application};


mod game;
mod entity;
mod utils;
mod params;
mod neurone;
mod brain;


fn main() -> iced::Result {
    // need Application to be run
    Game::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}