use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GameParameters {
    // Basics
    pub game_width: u16,
    pub game_height: u16,
    pub game_fps: u16,

    // Display
    // ...

    // Game Equilibrage
    pub land_seed: String,
    pub gravity: u64,

    // Game Timing
    pub dinausor_jump_velocity: f64,
    pub score_increase_speed_interval: f64,

    // Generation of New Obstacle
    pub min_obstacle_generation_time: f64,
    pub max_obstacle_generation_time: f64,
    pub obstacle_generation_time_decrease_speed: f64,
    pub obstacle_speed: f64,

    // Entity
    pub dinausor_width: u16,
    pub dinausor_height: u16,
    pub dinausor_x: f64,
    pub cactus_width: u16,
    pub cactus_height: u16,
    pub rock_width: u16,
    pub rock_height: u16,
    pub pterodactyle_width: u16,
    pub pterodactyle_height: u16,
    pub pterodactyle_flying_height: u16,
    pub pterodactyle_offset: u16,

    // Neurone
    pub brain_seed: String,
    pub neurone_width: u16,
    pub neurone_height: u16,
    pub neurone_web_creation_nb_neurones_min: u16,
    pub neurone_web_creation_nb_neurones_max: u16,
    pub brain_creation_nb_neurone_web_min: u16,
    pub brain_creation_nb_neurone_web_max: u16,
    pub neurone_web_add_mutation_rate: f64,
    pub neurone_web_remove_mutation_rate: f64,
    pub neurone_add_mutation_rate: f64,
    pub neurone_remove_mutation_rate: f64,
    pub neurone_x_mutation_range: f64,
    pub neurone_y_mutation_range: f64,

    // training
    pub training_nb_generation: u64,
    pub training_nb_brain: u64,
    /// the score limit to stop the training (if the brain reach this score, we actually consider it as a good brain)
    pub limit_score: u64,
    pub result_folder_path: String,
    /// the number of brain to save at the end of the training, if < 0 we save all the brain
    pub max_nb_brain_to_save: i64,
    /// the interval to save the result (in number of generation)
    pub interval_to_save_result: u64,
    // energie cost
    pub neuron_cost_mult: u64,
    pub neuron_web_cost_mult: u64,
    pub neuron_web_cost_flat: u64,
}


impl GameParameters {

    fn new_default() -> Self {
        GameParameters {
            // Basics
            game_width: 1280,
            game_height: 720,
            game_fps: 600,

            // Display
            // ...

            // Game Equilibrage
            land_seed: "42".to_string(),
            gravity: 2000,

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
            cactus_height: 80,
            rock_width: 40,
            rock_height: 40,
            pterodactyle_width: 40,
            pterodactyle_height: 40,
            pterodactyle_flying_height: 110,
            pterodactyle_offset: 130,

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
            neurone_add_mutation_rate: 0.2,
            neurone_remove_mutation_rate: 0.2,
            neurone_x_mutation_range: 500.0,
            neurone_y_mutation_range: 500.0,

            // training
            training_nb_generation: 1000,
            training_nb_brain: 500,
            limit_score: 500,
            result_folder_path: "./ressources/results/".to_string(),
            max_nb_brain_to_save: 100,
            interval_to_save_result: 10,
            // energie cost
            neuron_cost_mult: 5,
            neuron_web_cost_mult: 15,
            neuron_web_cost_flat : 10000,
            
        }
    }
}


lazy_static! {
    pub static ref PARAMS: GameParameters = GameParameters::new_default();
}