use rand::{Rng, distributions::Uniform};
use rand_pcg::Pcg64;
use serde::{Serialize, Deserialize};

use crate::{params::{PARAMS}, entity::Obstacle, utils::{check_collision, get_random_float, remove_indexes}};


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Neurone {
    pub x : f64,
    pub y : f64,
    pub width : u16,
    pub height : u16,
    
    pub activation_condition : NeuroneActivationCondition,
    pub activation : NeuroneActivation,
}

/// Neurone activation condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuroneActivationCondition {
    Air,
    Obstacle,
}

/// if the activation, force to not jump or jump
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuroneActivation {
    Jump,
    NoJump,
}

impl Neurone {
    /// create a totally new neurone
    pub fn new(x : f64, y : f64, activation_condition : NeuroneActivationCondition, activation : NeuroneActivation) -> Self {
        Self {
            x,
            y,
            width : (*PARAMS).neurone_width,
            height : (*PARAMS).neurone_height,
            activation_condition,
            activation,
        }
    }

    /// create a totaly new random neurone
    pub fn new_random(rng : &mut Pcg64) -> Self {
        let x = get_random_float(0.0, ((*PARAMS).game_width - (*PARAMS).neurone_width) as f64, rng);
        let y = get_random_float((*PARAMS).hole_height as f64 + 1.0, ((*PARAMS).game_height - (*PARAMS).neurone_height) as f64, rng);
        let activation_condition = 
            match rng.gen_range(0..2) {
                0 => NeuroneActivationCondition::Air,
                _ => NeuroneActivationCondition::Obstacle,
            };
        let activation =
            match rng.gen_range(0..2) {
                0 => NeuroneActivation::Jump,
                _ => NeuroneActivation::NoJump,
            };
        Self {
            x,
            y,
            width : (*PARAMS).neurone_width,
            height : (*PARAMS).neurone_height,
            activation_condition,
            activation,
        }
    }

    /// mutate this neurone
    pub fn mutate(&mut self, rng : &mut Pcg64) {
        // get the range of the mutation for x and y (we don't want to go out of the screen)
        let min_x = (self.x - (*PARAMS).neurone_x_mutation_range).max(0.0);
        let max_x = (self.x + (*PARAMS).neurone_x_mutation_range).min(((*PARAMS).game_width - (*PARAMS).neurone_width) as f64);
        let min_y = (self.y - (*PARAMS).neurone_y_mutation_range).max((*PARAMS).hole_height as f64 + 1.0);// we don't want to go under the hole
        let max_y = (self.y + (*PARAMS).neurone_y_mutation_range).min(((*PARAMS).game_height - (*PARAMS).neurone_height) as f64);

        self.x = get_random_float(min_x, max_x, rng);
        self.y = get_random_float(min_y, max_y, rng);
    }

    /// get the activation of the neurone if its condition is met
    pub fn get_activation(&self, obstacles : &Vec<Obstacle>) -> Option<NeuroneActivation> {
        let mut is_colision = false;
        for obstacle in obstacles {
            if check_collision(
                self.x,
                self.y,
                self.width,
                self.height,
                obstacle.x,
                obstacle.y,
                obstacle.width,
                obstacle.height,
            ) {
                // if the neurone is activated by an obstacle
                if self.activation_condition == NeuroneActivationCondition::Obstacle { 
                    return Some(self.activation.clone());
                }
                is_colision = true;
                break;
            }
        }
        // if the neurone is activated by air and there is no colision
        if !is_colision && self.activation_condition == NeuroneActivationCondition::Air {
            return Some(self.activation.clone());
        }else{
            return None;
        }
    }

    /// get the energy of the neurone
    pub fn get_energy(&self) -> f64 {
        // calcul the euclidian distance between the neurone and the dinausor
        let dinausor_x = (*PARAMS).dinausor_x + (*PARAMS).dinausor_width as f64 / 2.0;
        let dinausor_y = (*PARAMS).dinausor_height as f64 / 2.0;
        let neurone_x = self.x + (*PARAMS).neurone_width as f64 / 2.0;
        let neurone_y = self.y + (*PARAMS).neurone_height as f64 / 2.0;
        let distance = ((dinausor_x - neurone_x).powi(2) + (dinausor_y - neurone_y).powi(2)).sqrt();
        distance * (*PARAMS).neuron_cost_mult as f64 + (*PARAMS).neuron_cost_flat as f64    
    }

}

/// a web of neurone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuroneWeb {
    pub neurones : Vec<Neurone>,
}

impl NeuroneWeb {

    /// create a new completly random web of neurone
    pub fn new_random(rng : &mut Pcg64) -> Self {
        let mut neurones = Vec::new();
        // get the number of neurones
        let nb_neurones = rng.gen_range((*PARAMS).neurone_web_creation_nb_neurones_min..(*PARAMS).neurone_web_creation_nb_neurones_max);
        // gain of performance by declaring the distribution outside of the loop
        let x_dist = Uniform::from(0.0..((*PARAMS).game_width - (*PARAMS).neurone_width) as f64);
        let y_dist = Uniform::from(0.0..((*PARAMS).game_height - (*PARAMS).neurone_height) as f64);
        for _ in 0..nb_neurones {
            let x = rng.sample(x_dist);
            let y = rng.sample(y_dist);
            let activation_condition = 
                match rng.gen_range(0..2) {
                    0 => NeuroneActivationCondition::Air,
                    _ => NeuroneActivationCondition::Obstacle,
                };
            let activation =
                match rng.gen_range(0..2) {
                    0 => NeuroneActivation::Jump,
                    _ => NeuroneActivation::NoJump,
                };
            neurones.push(Neurone::new(x, y, activation_condition, activation));
        }
        Self {
            neurones,
        }
    }

    /// mutate this neurone web
    pub fn mutate(&mut self, rng : &mut Pcg64) {
        let mut neurones_to_remove: Vec<usize> = Vec::new();
        // mutate the neurone web
        for (i, neurone) in &mut self.neurones.iter_mut().enumerate() {   
            if rng.gen_bool((*PARAMS).neurone_remove_mutation_rate) {
                neurones_to_remove.push(i);
            }else{
                neurone.mutate(rng);
            }
        }

        // remove the marqued random web
        remove_indexes(&mut self.neurones, &neurones_to_remove);

        // add new neurone web if rng say so
        if rng.gen_bool((*PARAMS).neurone_web_add_mutation_rate) {
            self.neurones.push(Neurone::new_random(rng));
        }
    }


    pub fn is_jump(&self, obstacles : &Vec<Obstacle>) -> bool {
        let mut jump = false;
        for neurone in &self.neurones {
            // get the activation of the neurone
            let activation = neurone.get_activation(obstacles);
            // if the neurone is activated, we check if it is a force to not jump
            if activation.is_some() {
                if activation.unwrap() == NeuroneActivation::NoJump { // force the not jump
                    return false;
                }else{
                    // if we have a jump and no force to not jump, we can jump
                    jump = true;
                }
            }
        }

        jump
    }

    /// get the energy of the neurone web
    pub fn get_energy(&self) -> f64 {
        let mut energy = 0.0;
        for neurone in &self.neurones {
            energy += neurone.get_energy();
        }
        energy * (*PARAMS).neuron_web_cost_mult as f64 + (*PARAMS).neuron_web_cost_flat as f64
    }


}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::utils::str_to_u8_array;

    use super::*;

    #[test]
    fn test_neurone_generation(){
        for _ in 0..100 {
            let mut rng = Pcg64::from_seed([0; 32]);
            let neurone = Neurone::new_random(&mut rng);
            assert!(neurone.x >= 0.0);
            assert!(neurone.x <= ((*PARAMS).game_width - (*PARAMS).neurone_width) as f64);
            assert!(neurone.y >= 0.0);
            assert!(neurone.y <= ((*PARAMS).game_height - (*PARAMS).neurone_height) as f64);
        }        
    }

    #[test]
    fn test_neurone_web_generation(){
        for _ in 0..100 {
            let mut rng = Pcg64::from_seed(str_to_u8_array((*PARAMS).brain_seed.as_str()));
            let neurone_web = NeuroneWeb::new_random(&mut rng);
            assert!(neurone_web.neurones.len() >= (*PARAMS).neurone_web_creation_nb_neurones_min as usize);
            assert!(neurone_web.neurones.len() <= (*PARAMS).neurone_web_creation_nb_neurones_max as usize);
            for neurone in &neurone_web.neurones {
                assert!(neurone.x >= 0.0);
                assert!(neurone.x <= ((*PARAMS).game_width - (*PARAMS).neurone_width) as f64);
                assert!(neurone.y >= 0.0);
                assert!(neurone.y <= ((*PARAMS).game_height - (*PARAMS).neurone_height) as f64);
            }        
        }
    }

}