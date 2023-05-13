use std::path::{Path};
use std::time::{Instant, Duration};
use std::thread;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::utils::remove_indexes;
use crate::{neurone::NeuroneWeb, entity::Obstacle, params::{PARAMS}, utils::str_to_u8_array, game::Game};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brain {
    pub neurone_web : Vec<NeuroneWeb>
}

impl Brain {
    pub fn new(rng : &mut ChaChaRng) -> Self {
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
    pub fn mutate(&self, rng : &mut ChaChaRng) -> Self {
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
}


fn brain_run(brain : Brain) -> u64 {
    // create the game
    let mut now = Instant::now();
    let mut game = Game::new(now, &(*PARAMS).land_seed, Some(brain), None);
    let mut current_miliseconds = 0;
    let interval = 1000 / (*PARAMS).game_fps as u64;// in miliseconds
    // run the game
    while !game.has_lost && (game.score < (*PARAMS).limit_score) {
        current_miliseconds += interval;
        let duration = Duration::from_millis(current_miliseconds);
        now = now.checked_add(duration).unwrap();
        game.update(now);
    }

    game.score
}

/// train the brain
pub fn brain_train_pipeline(folder_path_input : Option<String>){
    // ----------- create the folder where we will save the brains ------------
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

    // Create the folder if it doesn't exist
    if !Path::new(&folder_path).exists() {
        fs::create_dir(folder_path.clone())
            .expect("Failed to create folder");
    }
    
    // save the params
    let params = serde_json::to_string(&(*PARAMS)).unwrap();
    let params_path = format!("{}params.json", folder_path.clone());
    fs::write(params_path, params).expect("Unable to write file");

    // ----------- create the brains ------------
    let mut rng = ChaChaRng::from_seed(str_to_u8_array((*PARAMS).brain_seed.as_str()));

    // create a lot of brain
    let mut brains = Vec::new();
    for _ in 0..(*PARAMS).training_nb_brain {
        brains.push(Brain::new(&mut rng));
    }

    for i in 0..(*PARAMS).training_nb_generation {
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

        // save the best brains
        let brain_str = serde_json::to_string(&best_brains).unwrap();
        let brain_path = format!("{}brain{}.json", folder_path.clone(), i);
        fs::write(brain_path, brain_str).expect("Unable to write file");

        println!("(it : {}) best score : {}", i, best_brains.get(0).unwrap().1);

        // genere the next generation (mutate all the best brains, begin randomly)
        // keep all the best brains and doesn't discard them
        // take a random brain from the best brains and rotate the index
        let mut best_scores_index = rng.gen_range(0..best_brains.len());
        let mut next_generation = Vec::new();
        for _ in 0..(*PARAMS).training_nb_brain {
            // mutate the brain
            let brain = best_brains[best_scores_index].0.mutate(&mut rng);
            next_generation.push(brain);
            // rotate the best scores index
            best_scores_index = best_scores_index + 1;
            if best_scores_index >= best_brains.len() {
                best_scores_index = 0;
            }
        }
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
        brains = next_generation;
    }

}