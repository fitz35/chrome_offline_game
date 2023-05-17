use iced::Color;
use rand::{Rng, distributions::Uniform};
use rand_pcg::Pcg64;
use serde::{Serialize, Deserialize};

use crate::{entity::Obstacle, utils::{check_collision, get_random_float, remove_indexes}, params::GameParameters};


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
    Activate,
    PreventActivate,
}

impl Neurone {
    /// create a totally new neurone
    pub fn new(params : &GameParameters, x : f64, y : f64, activation_condition : NeuroneActivationCondition, activation : NeuroneActivation) -> Self {
        Self {
            x,
            y,
            width : params.neurone_width,
            height : params.neurone_height,
            activation_condition,
            activation,
        }
    }

    /// create a totaly new random neurone
    pub fn new_random(params : &GameParameters, rng : &mut Pcg64) -> Self {
        let x = get_random_float(0.0, (params.game_width - params.neurone_width) as f64, rng);
        let y = get_random_float(params.hole_height as f64 + 5.0, (params.game_height - params.neurone_height) as f64, rng);
        let activation_condition = 
            match rng.gen_range(0..2) {
                0 => NeuroneActivationCondition::Air,
                _ => NeuroneActivationCondition::Obstacle,
            };
        let activation =
            match rng.gen_range(0..2) {
                0 => NeuroneActivation::Activate,
                _ => NeuroneActivation::PreventActivate,
            };
        Self {
            x,
            y,
            width : params.neurone_width,
            height : params.neurone_height,
            activation_condition,
            activation,
        }
    }

    /// mutate this neurone
    pub fn mutate(&mut self, params : &GameParameters, rng : &mut Pcg64) {
        // get the range of the mutation for x and y (we don't want to go out of the screen)
        let min_x = (self.x - params.neurone_x_mutation_range).max(0.0);
        let max_x = (self.x + params.neurone_x_mutation_range).min((params.game_width - params.neurone_width) as f64);
        let min_y = (self.y - params.neurone_y_mutation_range).max(params.hole_height as f64 + 5.0);// we don't want to go under the hole
        let max_y = (self.y + params.neurone_y_mutation_range).min((params.game_height - params.neurone_height) as f64);

        self.x = get_random_float(min_x, max_x, rng);
        self.y = get_random_float(min_y, max_y, rng);

        // mutate the activation condition if rng say so
        if rng.gen_bool(params.neurone_change_action_mutation_rate) {
            self.activation_condition = 
                match rng.gen_range(0..2) {
                    0 => NeuroneActivationCondition::Air,
                    _ => NeuroneActivationCondition::Obstacle,
                };
        }

        // mutate the activation if rng say so
        if rng.gen_bool(params.neurone_change_action_mutation_rate) {
            self.activation = 
                match rng.gen_range(0..2) {
                    0 => NeuroneActivation::Activate,
                    _ => NeuroneActivation::PreventActivate,
                };
        }
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
    pub fn get_energy(&self, params : &GameParameters) -> f64 {
        // calcul the euclidian distance between the neurone and the dinausor
        let dinausor_x = params.dinausor_x + params.dinausor_width as f64 / 2.0;
        let dinausor_y = params.dinausor_height as f64 / 2.0;
        let neurone_x = self.x + params.neurone_width as f64 / 2.0;
        let neurone_y = self.y + params.neurone_height as f64 / 2.0;
        let distance = ((dinausor_x - neurone_x).powi(2) + (dinausor_y - neurone_y).powi(2)).sqrt();
        distance * params.neuron_cost_mult as f64 + params.neuron_cost_flat as f64    
    }

    pub fn get_color(&self) -> Color {
        let alpha = 
            match self.activation_condition {
                NeuroneActivationCondition::Air => 0.5,
                NeuroneActivationCondition::Obstacle => 1.0,
            };

        match self.activation {
            NeuroneActivation::Activate => Color { r : 0.0, g : 1.0, b : 0.0, a : alpha},
            NeuroneActivation::PreventActivate => Color { r : 1.0, g : 0.0, b : 0.0, a : alpha}
        }
    }

}

/// action of the neurone web
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum NeuroneWebAction {
    Jump,
    Bend,
    Unbend
}

/// a web of neurone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuroneWeb {
    pub neurones : Vec<Neurone>,
    pub action : NeuroneWebAction,
}

impl NeuroneWeb {

    /// create a new completly random web of neurone
    pub fn new_random(params : &GameParameters, rng : &mut Pcg64) -> Self {
        let mut neurones = Vec::new();
        // get the number of neurones
        let nb_neurones = rng.gen_range(params.neurone_web_creation_nb_neurones_min..params.neurone_web_creation_nb_neurones_max);
        // gain of performance by declaring the distribution outside of the loop
        let x_dist = Uniform::from(0.0..(params.game_width - params.neurone_width) as f64);
        let y_dist = Uniform::from((params.hole_height as f64 + 5.0)..(params.game_height - params.neurone_height) as f64);
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
                    0 => NeuroneActivation::Activate,
                    _ => NeuroneActivation::PreventActivate,
                };
            neurones.push(Neurone::new(params, x, y, activation_condition, activation));
        }
        // get the action of the neurone web
        let action_i = rng.gen_range(0..params.commands.len());
        let action = params.commands[action_i].clone();

        Self {
            neurones,
            action,
        }
    }

    /// mutate this neurone web
    pub fn mutate(&mut self, params : &GameParameters, rng : &mut Pcg64) {
        let mut neurones_to_remove: Vec<usize> = Vec::new();
        // mutate the neurone web
        for (i, neurone) in &mut self.neurones.iter_mut().enumerate() {   
            if rng.gen_bool(params.neurone_remove_mutation_rate) {
                neurones_to_remove.push(i);
            }else{
                neurone.mutate(params, rng);
            }
        }

        // remove the marqued random web
        remove_indexes(&mut self.neurones, &neurones_to_remove);

        // add new neurone if rng say so
        if rng.gen_bool(params.neurone_web_add_mutation_rate) {
            self.neurones.push(Neurone::new_random(params, rng));
        }

        // mutate the action if rng say so
        if rng.gen_bool(params.neurone_web_change_action_mutation_rate) {
            let commands_i = rng.gen_range(0..params.commands.len());
            self.action = params.commands[commands_i].clone();
        }
    }


    pub fn is_activated(&self, obstacles : &Vec<Obstacle>) -> bool {
        let mut active = false;
        for neurone in &self.neurones {
            // get the activation of the neurone
            let activation = neurone.get_activation(obstacles);
            // if the neurone is activated, we check if it is a force to not jump
            if activation.is_some() {
                if activation.unwrap() == NeuroneActivation::PreventActivate { // force the not jump
                    return false;
                }else{
                    // if we have a jump and no force to not jump, we can jump
                    active = true;
                }
            }
        }

        active
    }

    /// get the energy of the neurone web
    pub fn get_energy(&self, params : &GameParameters) -> f64 {
        let mut energy = 0.0;
        for neurone in &self.neurones {
            energy += neurone.get_energy(params);
        }
        energy * params.neuron_web_cost_mult as f64 + params.neuron_web_cost_flat as f64
    }


}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::{utils::str_to_u8_array};

    use super::*;

    #[test]
    fn test_neurone_generation(){
        let params = GameParameters::new_default();
        for _ in 0..100 {
            let mut rng = Pcg64::from_seed([0; 32]);
            let neurone = Neurone::new_random(&params, &mut rng);
            assert!(neurone.x >= 0.0);
            assert!(neurone.x <= (params.game_width - params.neurone_width) as f64);
            assert!(neurone.y >= 0.0);
            assert!(neurone.y <= (params.game_height - params.neurone_height) as f64);
        }        
    }

    #[test]
    fn test_neurone_web_generation(){
        let params = GameParameters::new_default();
        for _ in 0..100 {
            let mut rng = Pcg64::from_seed(str_to_u8_array(params.brain_seed.as_str()));
            let neurone_web = NeuroneWeb::new_random(&params, &mut rng);
            assert!(neurone_web.neurones.len() >= params.neurone_web_creation_nb_neurones_min as usize);
            assert!(neurone_web.neurones.len() <= params.neurone_web_creation_nb_neurones_max as usize);
            for neurone in &neurone_web.neurones {
                assert!(neurone.x >= 0.0);
                assert!(neurone.x <= (params.game_width - params.neurone_width) as f64);
                assert!(neurone.y >= 0.0);
                assert!(neurone.y <= (params.game_height - params.neurone_height) as f64);
            }        
        }
    }

}