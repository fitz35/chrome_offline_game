use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Chrome training mode", about = "A program to train a dinausor to play chrome dino game")]
pub struct ProgramArgs {
    #[structopt(short = "t", long = "train", help = "Train the brain in the given folder", conflicts_with = "play_brain", conflicts_with = "play")]
    pub folder_path: Option<String>,

    #[structopt(short = "b", long = "play_brain", help = "Play the game with the given brain", conflicts_with = "train", conflicts_with = "play")]
    pub brain_path: Option<String>,

    #[structopt(short = "p", long = "play", help = "Play the game", conflicts_with = "train", conflicts_with = "play_brain")]
    pub play : bool,
}