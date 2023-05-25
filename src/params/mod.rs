use std::vec;

use serde::{Serialize, Deserialize};

use crate::{neurone::NeuroneWebAction, entity::ObstacleGenerateType};


pub const TRAINING_NB_GENERATION: u64 = 3_000_000;
/// the score limit to stop the training (if the brain reach this score, we actually consider it as a good brain)
pub const LIMIT_SCORE: u64 = 400;
/// the interval to save the result (in number of generation)
pub const INTERVAL_TO_SAVE_RESULT: u64 = 100;
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
    /// the obstacle that can be generated (depending on the commands, calculated at the start of the program)
    #[serde(skip)]
    pub obstacle_generate_types: Vec<ObstacleGenerateType>,

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
    
    // energie cost
    pub neuron_cost_mult: u64,
    pub neuron_cost_flat: u64,
    pub neuron_web_cost_mult: u64,
    pub neuron_web_cost_flat: u64,
    /// terrain generation (None : the seed is set to the param_land_seed, Some(5) : the seed is random every 5 generation)
    pub terrain_seed_generation_interval: Option<u64>,
}


impl GameParameters {

    pub fn new_default() -> Self {
        let mut params = GameParameters {
            // Basics
            game_width: 1280,
            game_height: 720,
            game_fps: 60,

            // Display
            // ...

            // Game Equilibrage
            land_seed: "gra".to_string(),
            gravity: 2000,
            commands: vec![NeuroneWebAction::Jump/*, NeuroneWebAction::Bend, NeuroneWebAction::Unbend*/],
            obstacle_generate_types: Vec::new(),

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
            pterodactyle_width: 120,
            pterodactyle_height: 40,
            pterodactyle_flying_height_with_rock: 110,
            pterodactyle_offset_with_rock: 65,
            pterodactyle_flying_height_without_rock: 95,
            hole_width: 90,
            hole_height: 2,

            // Neurone
            brain_seed: "Intellect".to_string(),
            neurone_width: 20,
            neurone_height: 20,

            neurone_web_creation_nb_neurones_min: 2,
            neurone_web_creation_nb_neurones_max: 6,
            brain_creation_nb_neurone_web_min: 1,
            brain_creation_nb_neurone_web_max: 3,

            neurone_web_add_mutation_rate: 0.4,
            neurone_web_remove_mutation_rate: 0.4,
            neurone_web_change_action_mutation_rate: 0.001,

            neurone_add_mutation_rate: 0.4,
            neurone_remove_mutation_rate: 0.4,
            neurone_change_action_mutation_rate: 0.001,

            neurone_x_mutation_range: 50.0,
            neurone_y_mutation_range: 50.0,

            // training
            training_nb_brain: 1000,
            max_nb_brain_to_save: 50,
            // energie cost
            neuron_cost_mult: 5,
            neuron_cost_flat : 100000,
            neuron_web_cost_mult: 15,
            neuron_web_cost_flat : 1000000,

            terrain_seed_generation_interval : None,
            
        };

        params.obstacle_generate_types = params.get_obstacles_generation_type();

        params
    }

    pub fn new_from_file(path: &str) -> Self {
        let file = std::fs::File::open(path).expect("Unable to open parameter file");
        let reader = std::io::BufReader::new(file);

        let mut params : GameParameters = serde_json::from_reader(reader).expect("Unable to parse parameter file");
        params.obstacle_generate_types = params.get_obstacles_generation_type();
        params

    }

    /// vget the possible obstacles that can be generated given the params
    /// WARN : not use HashMAp, it doesn't garantee the order, so the randomly repeatability is not garantee
    fn get_obstacles_generation_type(&self) -> Vec<ObstacleGenerateType> {
        let mut vector = Vec::new();
        let commands = &self.commands;
        // jump obstacle
        if commands.contains(&NeuroneWebAction::Jump) {
            vector.push(ObstacleGenerateType::Cactus);
            vector.push(ObstacleGenerateType::Rock);
            vector.push(ObstacleGenerateType::RockAndPterodactyle);
            vector.push(ObstacleGenerateType::RockAndHole);
        }
        // bend obstacle
        if commands.contains(&NeuroneWebAction::Bend) &&
            commands.contains(&NeuroneWebAction::Unbend) {
            vector.push(ObstacleGenerateType::Pterodactyle);
        }

        vector
    }
}