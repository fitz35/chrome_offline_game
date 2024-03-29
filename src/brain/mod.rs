use std::collections::HashSet;
use std::path::{Path};
use std::time::{Instant, Duration};
use std::thread;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::neurone::NeuroneWebAction;
use crate::params::{GameParameters, self};
use crate::utils::{remove_indexes, get_max_i};
use crate::{neurone::NeuroneWeb, entity::Obstacle, utils::str_to_u8_array, game::Game};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brain {
    pub neurone_web : Vec<NeuroneWeb>
}

impl Brain {
    pub fn new(params : &GameParameters, rng : &mut Pcg64) -> Self {
        let nb_neurone_web = rng.gen_range(params.brain_creation_nb_neurone_web_min..params.brain_creation_nb_neurone_web_max);
        let mut neurone_web = Vec::new();
        for _ in 0..nb_neurone_web {
            neurone_web.push(NeuroneWeb::new_random(params, rng));
        }
        
        Self {
            neurone_web
        }
    }

    /// mutate the brain and return a new brain
    pub fn mutate(&self, params : &GameParameters, rng : &mut Pcg64) -> Self {
        let mut new_neurone_web = self.neurone_web.clone();
        let mut neurone_to_remove: Vec<usize> = Vec::new();
        // mutate the neurone web
        for (i, neurone_web) in &mut new_neurone_web.iter_mut().enumerate() {    
            if rng.gen_bool(params.neurone_web_remove_mutation_rate) {
                neurone_to_remove.push(i);
            }else{
                neurone_web.mutate(params, rng);
            }
        }

        // remove the marqued random web
        remove_indexes(&mut new_neurone_web, &neurone_to_remove);

        // add new neurone web if rng say so
        if rng.gen_bool(params.neurone_web_add_mutation_rate) {
            new_neurone_web.push(NeuroneWeb::new_random(params, rng));
        }

        Self {
            neurone_web : new_neurone_web
        }
    }

    /// get the actions of the brain
    pub fn get_activations(&self, obstacles : &Vec<Obstacle>) -> HashSet<NeuroneWebAction> {
        let mut activations = HashSet::new();
        for neurone_web in &self.neurone_web {
            if neurone_web.is_activated(obstacles) {
                activations.insert(neurone_web.action.clone());
            }
        }

        activations
    }

    /// get the energie of the brain
    pub fn get_energie(&self, params : &GameParameters) -> f64 {
        let mut energie = 0.0;
        for neurone_web in &self.neurone_web {
            energie += neurone_web.get_energy(params);
        }

        energie
    }

    /// mutate a vect of brain into a number of brain
    /// genere the next generation (mutate all the best brains, begin randomly)
    /// keep all the best brains and doesn't discard them
    /// take a random brain from the best brains and rotate the index
    pub fn mutate_all(params : &GameParameters, brains : &Vec<Brain>, rng : &mut Pcg64) -> Vec<Brain> {
        let mut i = rng.gen_range(0..brains.len());
        let mut next_brains = Vec::new();
        for _ in 0..params.training_nb_brain {
            // mutate the brain
            let brain = brains[i].mutate(params, rng);
            next_brains.push(brain);
            // rotate the best scores index
            i = i + 1;
            if i >= brains.len() {
                i = 0;
            }
        }

        next_brains
    }
}


fn brain_run(params : &GameParameters, brain : Brain, seed : &str) -> u64 {
    // create the game
    let mut now = Instant::now();
    let mut game = Game::new(params, now, seed, Some(brain), None);
    let interval = 1000_000_000 / params.game_fps as u64;// in nanoseconds
    let duration = Duration::from_nanos(interval);
    // run the game
    while !game.has_lost && (game.score < params::LIMIT_SCORE) {
        now = now.checked_add(duration).unwrap();
        game.update(now);
    }

    game.score
}

/// generate the next generation
fn generate_next_generation(params : &GameParameters, ancestor : &Vec<Brain>, rng : &mut Pcg64) -> Vec<Brain> {
    let mut next_generation = Brain::mutate_all(params, &ancestor.iter().map(|brain| brain.clone()).collect(), rng);
    // add the old best brains randomly
    if params.max_nb_brain_to_save < 0 || ancestor.len() <= params.max_nb_brain_to_save as usize{
        for best_brain in ancestor {
            next_generation.push(best_brain.clone());
        }
    }else{
        let mut potential_i = Vec::new();
        for i in 0..ancestor.len() {
            potential_i.push(i);
        }

        for _ in 0..params.max_nb_brain_to_save as usize {
            // get a random index
            let index_potential_i = rng.gen_range(0..potential_i.len());
            let i_brain = potential_i.remove(index_potential_i);
            next_generation.push(ancestor[i_brain].clone());
            
        }
    }
    // update the next generation
    next_generation
}

/// wrapper
fn generate_next_generation_from_scoring(params : &GameParameters, ancestor : &Vec<&(Brain, u64)>, rng : &mut Pcg64)-> Vec<Brain> {
    generate_next_generation(params, &ancestor.iter().map(|&(brain, _)| brain.clone()).collect(), rng)
}

// generate the seed for the actual generation i
fn generate_seed(params : &GameParameters, i : u64, prec_seed : &str, rng : &mut Pcg64) -> String {
    match params.terrain_seed_generation_interval {
        Some(interval) => {
            if i % interval == 0 {
                rng
                    .sample_iter(rand::distributions::Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect()
            }else{
                prec_seed.to_string()
            }
        },
        None => {
            params.land_seed.as_str().to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntermediateResult {
    pub brains : Vec<Brain>,
    pub rng : Pcg64,
    pub score : u64,
}

/// train the brain
pub fn brain_train_pipeline(folder_path_input : Option<String>){
    // ----------- create the folder where we will save the brains (or load it) ------------
    let mut folder_path;
    if folder_path_input.is_some() {
        folder_path = folder_path_input.unwrap();
    }else{
        // Get the current timestamp
        let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to retrieve timestamp")
        .as_secs();

        // Convert the timestamp to a string
        let timestamp = timestamp.to_string();
        let result_folder = params::RESULT_FOLDER_PATH.clone();
        folder_path = format!("{}{}", result_folder, timestamp);
    }
    // check the slash at the end
    if !folder_path.ends_with("/") {
        folder_path = format!("{}/", folder_path);
    }

    let mut rng;
    let mut brains;
    let i_begin;
    let params; 

    // Create the folder if it doesn't exist
    if !Path::new(&folder_path).exists() {
        params = GameParameters::new_default();
        fs::create_dir(folder_path.clone())
            .expect("Failed to create folder");

        // save the params
        let params_str = serde_json::to_string(&params).unwrap();
        let params_path = format!("{}params.json", folder_path.clone());
        fs::write(params_path, params_str).expect("Unable to write file");

        rng = Pcg64::from_seed(str_to_u8_array(params.brain_seed.as_str()));

        // create a lot of brain
        brains = Vec::new();
        for _ in 0..params.training_nb_brain {
            brains.push(Brain::new(&params, &mut rng));
        }

        i_begin = 0;
    }else{
        // try to load the params and compare it to the current params
        let params_path = format!("{}params.json", folder_path.clone());
        params = GameParameters::new_from_file(&params_path);

        // get the last brain
        let max_i = get_max_i(&folder_path).expect("Unable to get the max i");
        let brain_path = format!("{}brain{}.json", folder_path.clone(), max_i);
        let result : IntermediateResult = serde_json::from_str(&fs::read_to_string(brain_path).expect("Unable to read file of result")).unwrap();
        
        rng = result.rng;
        brains = generate_next_generation(&params, &result.brains, &mut rng);

        i_begin = max_i + 1;
    }
    
    

    // ----------- run the brains ------------
    
    let mut land_seed = generate_seed(&params, 0, params.land_seed.as_str(), &mut rng);
    for i in i_begin..(i_begin + params::TRAINING_NB_GENERATION) {
        // run the brains in parallel
        println!("land seed : {}", land_seed);
        let mut scores : Vec<(Brain, u64)> = Vec::new();
        let mut handles = vec![];
        for brain in brains {
            let brain_copy = brain.clone();
            let seed_copy = land_seed.clone();
            let params_copy = params.clone();
            let handle = thread::spawn(move || {
                let result = brain_run(&params_copy, brain_copy, seed_copy.as_str());
                (brain, result)
            });
            handles.push(handle);
        }

        // wait for the threads to finish
        for handle in handles {
            let result = handle.join().unwrap();
            scores.push(result);
        }

        // ----------------- get the best brains -----------------
        // sort the scores
        let mut best_brains_score : Vec<&(Brain, u64)> = vec![&scores[0]];
        for score in &scores {
            if score.1 > best_brains_score.get(0).unwrap().1 {
                best_brains_score = vec![score];
            }else if score.1 == best_brains_score.get(0).unwrap().1 {
                best_brains_score.push(score);
            }
                
        }

        // sort on the energie if the score is max
        let mut best_brains: Vec<&(Brain, u64)>;
        if best_brains_score[0].1 == params::LIMIT_SCORE {
            best_brains = vec![best_brains_score[0]];
            for score in &best_brains_score {
                if score.0.get_energie(&params) < best_brains.get(0).unwrap().0.get_energie(&params) {
                    best_brains = vec![score];
                }else if score.0.get_energie(&params) == best_brains.get(0).unwrap().0.get_energie(&params) {
                    best_brains.push(score);
                }
            }
        }else{
            best_brains = best_brains_score;
        }


        // -------------------- save the progression (brain and random) -------------------------
        if i % params::INTERVAL_TO_SAVE_RESULT == 0 || i == i_begin + params::TRAINING_NB_GENERATION - 1{
            let brains_to_save: Vec<Brain> = best_brains.iter().map(|&(brain, _)| brain.clone()).collect();
            let to_save = IntermediateResult {
                brains : brains_to_save,
                rng : rng.clone(),
                score : best_brains[0].1,
            };
            // save the best brains
            let brain_str = serde_json::to_string(&to_save).unwrap();
            let brain_path = format!("{}brain{}.json", folder_path.clone(), i);
            fs::write(brain_path, brain_str).expect("Unable to write file");
        }
        

        println!("(it : {}) best score : {}, best energy : {}", i, best_brains.get(0).unwrap().1, best_brains.get(0).unwrap().0.get_energie(&params));

        // ------------------ create the next generation ------------------
        brains = generate_next_generation_from_scoring(&params, &best_brains, &mut rng);
        // get the seed
        land_seed = generate_seed(&params, i + 1, land_seed.as_str(), &mut rng);
        
    }

}