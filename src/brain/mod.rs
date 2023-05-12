use rand::{Rng};
use rand_chacha::ChaChaRng;

use crate::{neurone::NeuroneWeb, entity::Obstacle, params::{PARAMS}};

#[derive(Debug, Clone, PartialEq)]
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
        for i in neurone_to_remove {
            new_neurone_web.remove(i);
        }

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

