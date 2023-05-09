

use std::time::Instant;

use rand::Rng;
use rand_chacha::ChaChaRng;

use crate::params;

/// The different type of obstacle
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObstacleEntityType {
    Cactus = 0,
    Rock = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obstacle {
    /// bottom left point x
    pub x: i16,
    /// bottom left point y
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub velocity: u8,// horizontal px/s
    pub type_: ObstacleEntityType,
    pub last_time_update: Instant,
}

impl Obstacle {
    /// Create a new cactus obstacle
    pub fn new_cactus(x: i16, velocity : u8, last_time_update : Instant) -> Self {
        Self {
            x,
            y : 0,
            width : params::CACTUS_WIDTH,
            height : params::CACTUS_HEIGHT,
            velocity,
            type_: ObstacleEntityType::Cactus,
            last_time_update,
        }
    }

    /// Create a new rock obstacle
    pub fn new_rock(x: i16, velocity : u8, last_time_update : Instant) -> Self {
        Self {
            x,
            y : 0,
            width : params::ROCK_WIDTH,
            height : params::ROCK_HEIGHT,
            velocity,
            type_: ObstacleEntityType::Rock,
            last_time_update,
        }
    }

    /// wrapper to create an obstacle
    pub fn new(x: i16, velocity : u8, last_time_update : Instant, type_ : ObstacleEntityType) -> Self {
        match type_ {
            ObstacleEntityType::Cactus => Obstacle::new_cactus(x, velocity, last_time_update),
            ObstacleEntityType::Rock => Obstacle::new_rock(x, velocity, last_time_update)
        }
    }

    /// wrapper to create random obstacle
    pub fn new_random(x: i16, velocity : u8, last_time_update : Instant,  rng : &mut ChaChaRng) -> Self {
        let random_entity: ObstacleEntityType = match rng.gen_range(0..2) {
            0 => ObstacleEntityType::Cactus,
            _ => ObstacleEntityType::Rock,
        };
        Obstacle::new(x, velocity, last_time_update, random_entity)
    }

    /// Update the obstacle position
    pub fn update(&mut self, now: Instant) {
        let delta = now.duration_since(self.last_time_update).as_secs_f32();
        if delta < 0.0 {// if the time is in the future, we don't update
            return;
        }
        self.x -= (delta * self.velocity as f32) as i16;
        self.last_time_update = now;
    }

    //----------- internal helper/logic ------------
    
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dinosaur {
    /// bottom left point x
    pub x: i16,
    /// bottom left point y
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub velocity: i16,// vertical px/s
    pub last_time_update: Instant,// in seconds
}


impl Dinosaur {
    pub fn new_dinosaur(last_time_update : Instant) -> Self {
        Self {
            x : 50,
            y : 0,
            width : params::DINAUSOR_WIDTH,
            height : params::DINAUSOR_HEIGHT,
            velocity : 0,
            last_time_update,
        }
    }

    /// jump
    fn intern_hump(&mut self, velocity : i16) -> bool {
        if self.y == 0 && self.velocity == 0{
            self.velocity = velocity;
            return true;
        }
        false
    }

    /// rapid jump
    pub fn jump(&mut self) -> bool {
        self.intern_hump(20)
    }
        
    /// long jump
    pub fn long_jump(&mut self) -> bool {
        self.intern_hump(40)
    }

    /// Update the position and apply the gravity
    pub fn update(&mut self, tick: Instant) {
        let delta = (tick - self.last_time_update).as_secs_f32();
        self.y += (self.velocity as f32 * delta) as i16;
        self.velocity -= (params::GRAVITY as f32 * delta) as i16;
        if self.y <= 0 {
            self.y = 0;
            self.velocity = 0;
        }
        self.last_time_update = tick;
    }
}