use rand::{Rng, distributions::Uniform};
use rand_chacha::ChaChaRng;

use crate::{params, entity::Obstacle, utils::{check_collision, get_random_float}};


#[derive(Debug, Clone, PartialEq)]
pub struct Neurone {
    pub x : f64,
    pub y : f64,
    pub width : u16,
    pub height : u16,
    
    pub activation_condition : NeuroneActivationCondition,
    pub activation : NeuroneActivation,
}

/// Neurone activation condition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeuroneActivationCondition {
    Air,
    Obstacle,
}

/// if the activation, force to not jump or jump
#[derive(Debug, Clone, PartialEq, Eq)]
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
            width : params::NEURONE_WIDTH,
            height : params::NEURONE_HEIGHT,
            activation_condition,
            activation,
        }
    }

    /// create a totaly new random neurone
    pub fn new_random(rng : &mut ChaChaRng) -> Self {
        let x = get_random_float(0.0, (params::GAME_WIDTH - params::NEURONE_WIDTH) as f64, rng);
        let y = get_random_float(0.0, (params::GAME_HEIGHT - params::NEURONE_HEIGHT) as f64, rng);
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
            width : params::NEURONE_WIDTH,
            height : params::NEURONE_HEIGHT,
            activation_condition,
            activation,
        }
    }

    /// mutate this neurone
    pub fn mutate(&mut self, rng : &mut ChaChaRng) {
        self.x = get_random_float(self.x - params::NEURONE_X_MUTATION_RANGE, self.x + params::NEURONE_X_MUTATION_RANGE, rng);
        self.y = get_random_float(self.y - params::NEURONE_Y_MUTATION_RANGE, self.y + params::NEURONE_Y_MUTATION_RANGE, rng);
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

    // --------------------- random helper ---------------------

}

/// a web of neurone
#[derive(Debug, Clone, PartialEq)]
pub struct NeuroneWeb {
    pub neurones : Vec<Neurone>,
}

impl NeuroneWeb {
    /// create a new web of neurone
    pub fn new(neurones : Vec<Neurone>) -> Self {
        Self {
            neurones,
        }
    }

    /// create a new completly random web of neurone
    pub fn new_random(rng : &mut ChaChaRng) -> Self {
        let mut neurones = Vec::new();
        // get the number of neurones
        let nb_neurones = rng.gen_range(params::NEURONE_WEB_CREATION_NB_NEURONES_MIN..params::NEURONE_WEB_CREATION_NB_NEURONES_MAX);
        // gain of performance by declaring the distribution outside of the loop
        let x_dist = Uniform::from(0.0..(params::GAME_WIDTH - params::NEURONE_WIDTH) as f64);
        let y_dist = Uniform::from(0.0..(params::GAME_HEIGHT - params::NEURONE_HEIGHT) as f64);
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
    pub fn mutate(&mut self, rng : &mut ChaChaRng) {
        let mut neurones_to_remove: Vec<usize> = Vec::new();
        // mutate the neurone web
        for (i, neurone) in &mut self.neurones.iter_mut().enumerate() {    
            if rng.gen_bool(params::NEURONE_REMOVE_MUTATION_RATE) {
                neurones_to_remove.push(i);
            }else{
                neurone.mutate(rng);
            }
        }

        // remove the marqued random web
        for i in neurones_to_remove {
            self.neurones.remove(i);
        }

        // add new neurone web if rng say so
        if rng.gen_bool(params::NEURONE_WEB_ADD_MUTATION_RATE) {
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


}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::utils::str_to_u8_array;

    use super::*;

    #[test]
    fn test_neurone_generation(){
        for _ in 0..100 {
            let mut rng = ChaChaRng::from_seed([0; 32]);
            let neurone = Neurone::new_random(&mut rng);
            assert!(neurone.x >= 0.0);
            assert!(neurone.x <= (params::GAME_WIDTH - params::NEURONE_WIDTH) as f64);
            assert!(neurone.y >= 0.0);
            assert!(neurone.y <= (params::GAME_HEIGHT - params::NEURONE_HEIGHT) as f64);
        }        
    }

    #[test]
    fn test_neurone_web_generation(){
        for _ in 0..100 {
            let mut rng = ChaChaRng::from_seed(str_to_u8_array(params::BRAIN_SEED));
            let neurone_web = NeuroneWeb::new_random(&mut rng);
            assert!(neurone_web.neurones.len() >= params::NEURONE_WEB_CREATION_NB_NEURONES_MIN as usize);
            assert!(neurone_web.neurones.len() <= params::NEURONE_WEB_CREATION_NB_NEURONES_MAX as usize);
            for neurone in &neurone_web.neurones {
                assert!(neurone.x >= 0.0);
                assert!(neurone.x <= (params::GAME_WIDTH - params::NEURONE_WIDTH) as f64);
                assert!(neurone.y >= 0.0);
                assert!(neurone.y <= (params::GAME_HEIGHT - params::NEURONE_HEIGHT) as f64);
            }        
        }
    }

}