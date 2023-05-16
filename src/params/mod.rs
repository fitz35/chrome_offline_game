use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

use crate::neurone::NeuroneWebAction;


pub const TRAINING_NB_GENERATION: u64 = 150000;
/// the score limit to stop the training (if the brain reach this score, we actually consider it as a good brain)
pub const LIMIT_SCORE: u64 = 400;
pub const RESULT_FOLDER_PATH: &str = "./ressources/results/";

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GameParameters {
    // -------------- Basics ---------------
    pub game_width: u16,
    pub game_height: u16,
    pub game_fps: u16,

    // ---------------- Display -------------
    // ...

    // ----------------- Game Equilibrage -----------------
    pub land_seed: String,
    pub gravity: u64,
    /// the commands that the neurone web can do (and the obstacle generation)
    pub commands: Vec<NeuroneWebAction>,

    // ------------------- Game Timing --------------------
    pub dinausor_jump_velocity: f64,
    pub score_increase_speed_interval: f64,

    // ---------------  Generation of New Obstacle ---------------
    pub min_obstacle_generation_time: f64,
    pub max_obstacle_generation_time: f64,
    pub obstacle_generation_time_decrease_speed: f64,
    pub obstacle_speed: f64,

    // ------------------- Entity --------------------
    // dinausor
    pub dinausor_width: u16,
    pub dinausor_height: u16,
    pub dinausor_x: f64,
    // obstacle
    // cactus
    pub cactus_width: u16,
    pub cactus_height: u16,
    // rock
    pub rock_width: u16,
    pub rock_height: u16,
    // pterodactyle
    pub pterodactyle_width: u16,
    pub pterodactyle_height: u16,
    pub pterodactyle_flying_height_with_rock: u16,
    pub pterodactyle_offset_with_rock: u16,
    pub pterodactyle_flying_height_without_rock: u16,
    // hole
    pub hole_width: u16,
    /// the height of the hole must be taken in the limit of the neuron y (the neuron must not detect him !)
    pub hole_height: u16,

    // ------------------ Neurone -------------------
    pub brain_seed: String,

    pub neurone_width: u16,
    pub neurone_height: u16,

    pub neurone_web_creation_nb_neurones_min: u16,
    pub neurone_web_creation_nb_neurones_max: u16,
    pub brain_creation_nb_neurone_web_min: u16,
    pub brain_creation_nb_neurone_web_max: u16,

    pub neurone_web_add_mutation_rate: f64,
    pub neurone_web_remove_mutation_rate: f64,
    pub neurone_web_change_action_mutation_rate : f64,

    pub neurone_add_mutation_rate: f64,
    pub neurone_remove_mutation_rate: f64,
    pub neurone_change_action_mutation_rate : f64,

    pub neurone_x_mutation_range: f64,
    pub neurone_y_mutation_range: f64,

    //  ---------------- training -----------------
    pub training_nb_brain: u64,
    
    /// the number of brain to save at the end of the training, if < 0 we save all the brain
    pub max_nb_brain_to_save: i64,
    /// the interval to save the result (in number of generation)
    pub interval_to_save_result: u64,
    // energie cost
    pub neuron_cost_mult: u64,
    pub neuron_cost_flat: u64,
    pub neuron_web_cost_mult: u64,
    pub neuron_web_cost_flat: u64,
    /// terrain generation (None : the seed is set to the param_land_seed, Some(5) : the seed is random every 5 generation)
    pub terrain_seed_generation_interval: Option<u64>,
}


impl GameParameters {

    fn new_default() -> Self {
        GameParameters {
            // Basics
            game_width: 1280,
            game_height: 720,
            game_fps: 60,

            // Display
            // ...

            // Game Equilibrage
            land_seed: "42".to_string(),
            gravity: 2000,
            commands: vec![NeuroneWebAction::Jump],

            // Game Timing
            dinausor_jump_velocity: 800.0,
            score_increase_speed_interval: 2.0,

            // Generation of New Obstacle
            min_obstacle_generation_time: 1.2,
            max_obstacle_generation_time: 2.0,
            obstacle_generation_time_decrease_speed: 0.2,
            obstacle_speed: 400.0,

            // Entity
            dinausor_width: 40,
            dinausor_height: 100,
            dinausor_x: 50.0,
            cactus_width: 40,
            cactus_height: 100,
            rock_width: 40,
            rock_height: 40,
            pterodactyle_width: 85,
            pterodactyle_height: 40,
            pterodactyle_flying_height_with_rock: 110,
            pterodactyle_offset_with_rock: 140,
            pterodactyle_flying_height_without_rock: 95,
            hole_width: 60,
            hole_height: 2,

            // Neurone
            brain_seed: "Intellect".to_string(),
            neurone_width: 20,
            neurone_height: 20,

            neurone_web_creation_nb_neurones_min: 3,
            neurone_web_creation_nb_neurones_max: 6,
            brain_creation_nb_neurone_web_min: 1,
            brain_creation_nb_neurone_web_max: 3,

            neurone_web_add_mutation_rate: 0.1,
            neurone_web_remove_mutation_rate: 0.1,
            neurone_web_change_action_mutation_rate: 0.0,

            neurone_add_mutation_rate: 0.2,
            neurone_remove_mutation_rate: 0.2,
            neurone_change_action_mutation_rate: 0.0,

            neurone_x_mutation_range: 500.0,
            neurone_y_mutation_range: 500.0,

            // training
            training_nb_brain: 6000,
            max_nb_brain_to_save: 100,
            interval_to_save_result: 100,
            // energie cost
            neuron_cost_mult: 5,
            neuron_cost_flat : 100000,
            neuron_web_cost_mult: 15,
            neuron_web_cost_flat : 1000000,

            terrain_seed_generation_interval : None,
            
        }
    }

    pub fn new_from_file(path: &str) -> Self {
        let file = std::fs::File::open(path).expect("Unable to open parameter file");
        let reader = std::io::BufReader::new(file);

        serde_json::from_reader(reader).expect("Unable to parse parameter file")
    }
}


lazy_static! {
    pub static ref PARAMS: GameParameters = GameParameters::new_default();
}