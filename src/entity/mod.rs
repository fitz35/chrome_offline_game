

use std::time::Instant;

use crate::params::{PARAMS};

/// The different type of obstacle
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObstacleEntityType {
    Cactus = 0,
    Rock = 1,
    Pterodactyle = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObstacleGenerateType {
    Cactus = 0,
    Rock = 1,
    RockAndPterodactyle = 2,
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
    pub fn new_cactus(x: f64, velocity : f64, last_time_update : Instant) -> Self {
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
    pub fn new_rock(x: f64, velocity : f64, last_time_update : Instant) -> Self {
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

    pub fn new_pterodactyle(x: f64, velocity : f64, last_time_update : Instant) -> Self {
        Self {
            x : x - (*PARAMS).pterodactyle_offset as f64,
            y : (*PARAMS).pterodactyle_flying_height as f64,
            width : (*PARAMS).pterodactyle_width,
            height : (*PARAMS).pterodactyle_height,
            velocity : velocity,
            type_: ObstacleEntityType::Pterodactyle,
            last_time_update,
        }
    }

    /// wrapper to create an obstacle
    pub fn new(x: f64, velocity : f64, last_time_update : Instant, type_ : ObstacleEntityType) -> Self {
        match type_ {
            ObstacleEntityType::Cactus => Obstacle::new_cactus(x, velocity, last_time_update),
            ObstacleEntityType::Rock => Obstacle::new_rock(x, velocity, last_time_update),
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
        //println!("update obstacle : {:?} -> {:?}", self, delta);
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
}


impl Dinosaur {
    pub fn new_dinosaur(last_time_update : Instant) -> Self {
        Self {
            x : 50.0,
            y : 0.0,
            width : (*PARAMS).dinausor_width,
            height : (*PARAMS).dinausor_height,
            velocity : 0.0,
            last_time_update,
        }
    }

    /// jump
    fn intern_hump(&mut self, velocity : f64) -> bool {
        if self.y <= 0.0 && self.velocity <= 0.0{
            self.velocity = velocity;
            return true;
        }
        false
    }

    /// rapid jump
    pub fn jump(&mut self) -> bool {
        self.intern_hump((*PARAMS).dinausor_jump_velocity as f64)
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