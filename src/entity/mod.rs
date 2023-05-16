

use std::{time::Instant};

use lazy_static::{lazy_static};

use crate::{params::{PARAMS}, neurone::NeuroneWebAction};

/// The different type of obstacle
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObstacleEntityType {
    Cactus = 0,
    Rock = 1,
    PterodactyleWithRock = 2,
    Hole = 3,
    Pterodactyle = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObstacleGenerateType {
    Cactus = 0,
    Rock = 1,
    RockAndPterodactyle = 2,
    RockAndHole = 3,
    Pterodactyle = 4,
}

lazy_static! {
    /// vget the possible obstacles that can be generated given the params
    /// WARN : not use HashMAp, it doesn't garantee the order, so the randomly repeatability is not garantee
    pub static ref OBSTACLE_GENERATE_TYPES: Vec<ObstacleGenerateType> = {
        let mut vector = Vec::new();
        let commands = &(*PARAMS).commands;
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
    };
}


#[derive(Debug, Clone, PartialEq)]
pub struct Obstacle {
    /// bottom left point x
    pub x: f64,
    /// bottom left point y
    pub y: f64,
    pub width: u16,
    pub height: u16,
    pub velocity: f64,// horizontal px/s
    pub type_: ObstacleEntityType,
    pub last_time_update: Instant,
}

impl Obstacle {
    /// Create a new cactus obstacle
    fn new_cactus(x: f64, velocity : f64, last_time_update : Instant) -> Self {
        Self {
            x,
            y : 0.0,
            width : (*PARAMS).cactus_width,
            height : (*PARAMS).cactus_height,
            velocity,
            type_: ObstacleEntityType::Cactus,
            last_time_update,
        }
    }

    /// Create a new rock obstacle
    fn new_rock(x: f64, velocity : f64, last_time_update : Instant) -> Self {
        Self {
            x,
            y : 0.0,
            width : (*PARAMS).rock_width,
            height : (*PARAMS).rock_height,
            velocity,
            type_: ObstacleEntityType::Rock,
            last_time_update,
        }
    }

    fn new_pterodactyle_with_rock(x: f64, velocity : f64, last_time_update : Instant) -> Self {
        Self {
            x : x - (*PARAMS).pterodactyle_offset_with_rock as f64 - (*PARAMS).pterodactyle_width as f64,
            y : (*PARAMS).pterodactyle_flying_height_with_rock as f64,
            width : (*PARAMS).pterodactyle_width,
            height : (*PARAMS).pterodactyle_height,
            velocity : velocity,
            type_: ObstacleEntityType::PterodactyleWithRock,
            last_time_update,
        }
    }

    fn new_pterodactyle(x: f64, velocity : f64, last_time_update : Instant) -> Self {
        Self {
            x,
            y : (*PARAMS).pterodactyle_flying_height_without_rock as f64,
            width : (*PARAMS).pterodactyle_width,
            height : (*PARAMS).pterodactyle_height,
            velocity : velocity,
            type_: ObstacleEntityType::Pterodactyle,
            last_time_update,
        }
    }

    fn new_hole(x: f64, velocity : f64, last_time_update : Instant) -> Self {
        Self {
            x : x - (*PARAMS).hole_width as f64,
            y : 0.0,
            width : (*PARAMS).hole_width,
            height : (*PARAMS).hole_height,
            velocity,
            type_: ObstacleEntityType::Hole,
            last_time_update,
        }
    }

    /// wrapper to create an obstacle
    pub fn new(x: f64, velocity : f64, last_time_update : Instant, type_ : ObstacleEntityType) -> Self {
        let x = x + 400.0;
        match type_ {
            ObstacleEntityType::Cactus => Obstacle::new_cactus(x, velocity, last_time_update),
            ObstacleEntityType::Rock => Obstacle::new_rock(x, velocity, last_time_update),
            ObstacleEntityType::PterodactyleWithRock => Obstacle::new_pterodactyle_with_rock(x, velocity, last_time_update),
            ObstacleEntityType::Hole => Obstacle::new_hole(x, velocity, last_time_update),
            ObstacleEntityType::Pterodactyle => Obstacle::new_pterodactyle(x, velocity, last_time_update),
        }
    }

    /// Update the obstacle position
    pub fn update(&mut self, now: Instant) {
        if now < self.last_time_update {// if the time is in the future, we don't update
            return;
        }

        let delta = now.duration_since(self.last_time_update).as_secs_f64();
       
        
        self.x -= delta * self.velocity;
        
        self.last_time_update = now;
    }

    //----------- internal helper/logic ------------
    
}


#[derive(Debug, Clone, PartialEq)]
pub struct Dinosaur {
    /// bottom left point x
    pub x: f64,
    /// bottom left point y
    pub y: f64,
    pub width: u16,
    pub height: u16,
    pub velocity: f64,// vertical px/s
    pub last_time_update: Instant,// in seconds

    pub is_bending : bool,
}


impl Dinosaur {
    pub fn new_dinosaur(last_time_update : Instant) -> Self {
        Self {
            x : (*PARAMS).dinausor_x,
            y : 0.0,
            width : (*PARAMS).dinausor_width,
            height : (*PARAMS).dinausor_height,
            velocity : 0.0,
            last_time_update,
            is_bending : false,
        }
    }

    /// jump 
    /// return true if the jump is done
    /// NOTE : Cant jump if it is bending
    fn intern_hump(&mut self, velocity : f64) -> bool {
        if self.y <= 0.0 && self.velocity <= 0.0 && !self.is_bending{
            self.velocity = velocity;
            return true;
        }
        false
    }

    /// rapid jump
    /// return true if the jump is done
    pub fn jump(&mut self) -> bool {
        self.intern_hump((*PARAMS).dinausor_jump_velocity as f64)
    }

    /// bend
    pub fn bend(&mut self) -> bool {
        if self.is_bending {
            return false;
        }else{
            self.is_bending = true;
            // switch height and width
            self.switch_width_height();
            return true;
        }
    }

    /// unbend
    pub fn unbend(&mut self) -> bool {
        if !self.is_bending {
            return false;
        }else{
            self.is_bending = false;
            // switch height and width
            self.switch_width_height();
            return true;
        }
    }

    fn switch_width_height(&mut self) {
        let buf = self.width;
        self.width = self.height;
        self.height = buf;
    }


    /// Update the position and apply the gravity
    pub fn update(&mut self, tick: Instant) {
        let delta = tick.duration_since(self.last_time_update).as_secs_f64();
        self.y += self.velocity * delta;
        self.velocity -= (*PARAMS).gravity as f64 * delta;
        if self.y <= 0.0 {
            self.y = 0.0;
            self.velocity = 0.0;
        }
        self.last_time_update = tick;
    }
}