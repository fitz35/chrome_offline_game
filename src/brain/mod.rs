use std::path::{Path};
use std::time::{Instant, Duration};
use std::thread;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::params::GameParameters;
use crate::utils::{remove_indexes, get_max_i};
use crate::{neurone::NeuroneWeb, entity::Obstacle, params::{PARAMS}, utils::str_to_u8_array, game::Game};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brain {
    pub neurone_web : Vec<NeuroneWeb>
}

impl Brain {
    pub fn new(rng : &mut Pcg64) -> Self {
        let nb_neurone_web = rng.gen_range((*PARAMS).brain_creation_nb_neurone_web_min..(*PARAMS).brain_creation_nb_neurone_web_max);
        let mut neurone_web = Vec::new();
        for _ in 0..nb_neurone_web {
            neurone_web.push(NeuroneWeb::new_random(rng));
        }
        
        Self {
            neurone_web
        }
    }

    /// mutate the brain and return a new brain
    pub fn mutate(&self, rng : &mut Pcg64) -> Self {
        let mut new_neurone_web = self.neurone_web.clone();
        let mut neurone_to_remove: Vec<usize> = Vec::new();
        // mutate the neurone web
        for (i, neurone_web) in &mut new_neurone_web.iter_mut().enumerate() {    
            if rng.gen_bool((*PARAMS).neurone_web_remove_mutation_rate) {
                neurone_to_remove.push(i);
            }else{
                neurone_web.mutate(rng);
            }
        }

        // remove the marqued random web
        remove_indexes(&mut new_neurone_web, &neurone_to_remove);

        // add new neurone web if rng say so
        if rng.gen_bool((*PARAMS).neurone_web_add_mutation_rate) {
            new_neurone_web.push(NeuroneWeb::new_random(rng));
        }

        Self {
            neurone_web : new_neurone_web
        }
    }



    /// ask the brain if the dinausor should jump
    pub fn is_jump(&self, obstacles : &Vec<Obstacle>) -> bool {
        for neurone_web in &self.neurone_web {
            if neurone_web.is_jump(obstacles) {
                return true;
            }
        }

        false
    }

    /// mutate a vect of brain into a number of brain
    /// genere the next generation (mutate all the best brains, begin randomly)
    /// keep all the best brains and doesn't discard them
    /// take a random brain from the best brains and rotate the index
    pub fn mutate_all(brains : &Vec<Brain>, rng : &mut Pcg64) -> Vec<Brain> {
        let mut i = rng.gen_range(0..brains.len());
        let mut next_brains = Vec::new();
        for _ in 0..(*PARAMS).training_nb_brain {
            // mutate the brain
            let brain = brains[i].mutate(rng);
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


fn brain_run(brain : Brain) -> u64 {
    // create the game
    let mut now = Instant::now();
    let mut game = Game::new(now, &(*PARAMS).land_seed, Some(brain), None);
    let mut current_miliseconds = 0;
    let interval = 1000_000_000 / (*PARAMS).game_fps as u64;// in miliseconds
    // run the game
    while !game.has_lost && (game.score < (*PARAMS).limit_score) {
        current_miliseconds += interval;
        let duration = Duration::from_nanos(current_miliseconds);
        now = now.checked_add(duration).unwrap();
        game.update(now);
    }

    game.score
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntermediateResult {
    pub brains : Vec<Brain>,
    pub rng : Pcg64,
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
        let result_folder = (*PARAMS).result_folder_path.clone();
        folder_path = format!("{}{}", result_folder, timestamp);
    }
    // check the slash at the end
    if !folder_path.ends_with("/") {
        folder_path = format!("{}/", folder_path);
    }

    let mut rng;
    let mut brains;
    let i_begin;

    // Create the folder if it doesn't exist
    if !Path::new(&folder_path).exists() {
        fs::create_dir(folder_path.clone())
            .expect("Failed to create folder");

        // save the params
        let params = serde_json::to_string(&(*PARAMS)).unwrap();
        let params_path = format!("{}params.json", folder_path.clone());
        fs::write(params_path, params).expect("Unable to write file");

        rng = Pcg64::from_seed(str_to_u8_array((*PARAMS).brain_seed.as_str()));

        // create a lot of brain
        brains = Vec::new();
        for _ in 0..(*PARAMS).training_nb_brain {
            brains.push(Brain::new(&mut rng));
        }

        i_begin = 0;
    }else{
        // try to load the params and compare it to the current params
        let params_path = format!("{}params.json", folder_path.clone());
        let params_str = fs::read_to_string(params_path).expect("Unable to read file params.json");
        let params_to_compare : GameParameters = serde_json::from_str(&params_str).unwrap();
        if params_to_compare != (*PARAMS) {
            println!("The params are different, we can't continue the training");
            return;
        }

        // get the last brain
        let max_i = get_max_i(&folder_path).expect("Unable to get the max i");
        let brain_path = format!("{}brain{}.json", folder_path.clone(), max_i);
        let result : IntermediateResult = serde_json::from_str(&fs::read_to_string(brain_path).expect("Unable to read file of result")).unwrap();
        
        rng = result.rng;
        brains = Brain::mutate_all(&result.brains, &mut rng);

        i_begin = max_i + 1;
    }
    
    

    // ----------- run the brains ------------
    

    for i in i_begin..(i_begin + (*PARAMS).training_nb_generation) {
        // run the brains in parallel
        let mut scores : Vec<(Brain, u64)> = Vec::new();
        let mut handles = vec![];
        for brain in brains {
            let brain_copy = brain.clone();
            let handle = thread::spawn(move || {
                let result = brain_run(brain_copy);
                (brain, result)
            });
            handles.push(handle);
        }

        // wait for the threads to finish
        for handle in handles {
            let result = handle.join().unwrap();
            scores.push(result);
        }

        // get the best brains
        let mut best_brains : Vec<&(Brain, u64)> = vec![&scores[0]];
        for score in &scores {
            if score.1 > best_brains.get(0).unwrap().1 {
                best_brains = vec![score];
            }else if score.1 == best_brains.get(0).unwrap().1 {
                best_brains.push(score);
            }
                
        }

        // save the progression (brain and random)
        if i % (PARAMS).interval_to_save_result == 0 || i == i_begin + (*PARAMS).training_nb_generation - 1{
            let brains_to_save: Vec<Brain> = best_brains.iter().map(|&(brain, _)| brain.clone()).collect();
            let to_save = IntermediateResult {
                brains : brains_to_save,
                rng : rng.clone()
            };
            // save the best brains
            let brain_str = serde_json::to_string(&to_save).unwrap();
            let brain_path = format!("{}brain{}.json", folder_path.clone(), i);
            fs::write(brain_path, brain_str).expect("Unable to write file");
        }
        

        println!("(it : {}) best score : {}", i, best_brains.get(0).unwrap().1);

        
        let mut next_generation = Brain::mutate_all(&best_brains.iter().map(|&(brain, _)| brain.clone()).collect(), &mut rng);
        // add the old best brains randomly
        if best_brains.len() <= (*PARAMS).max_nb_brain_to_save as usize ||
            (*PARAMS).max_nb_brain_to_save < 0 {
            for best_brain in best_brains {
                next_generation.push(best_brain.0.clone());
            }
        }else{
            for _ in 0..(*PARAMS).max_nb_brain_to_save {
                // get a random index
                let i_brain = rng.gen_range(0..best_brains.len());

                let brain = best_brains[i_brain].0.clone();
                next_generation.push(brain);
            }
        }
        // update the next generation
        brains = next_generation;
    }

}