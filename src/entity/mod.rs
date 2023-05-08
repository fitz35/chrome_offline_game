

use std::time::Instant;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObstacleType {
    Cactus,
    Rock,
    Pterodactyl,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obstacle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub velocity: u8,// horizontal px/s
    pub type_: ObstacleType,
    pub last_time_update: Instant,
}

impl Obstacle {
    /// Create a new pterodactyle obstacle
    pub fn new_pterodactyl(x: u16, y: u16, last_time_update: Instant) -> Self {
        Self {
            x,
            y,
            width : 50,
            height : 40,
            velocity : 20,
            type_: ObstacleType::Pterodactyl,
            last_time_update,
        }
    }

    /// Create a new cactus obstacle
    pub fn new_cactus(x: u16, y: u16, last_time_update : Instant) -> Self {
        Self {
            x,
            y,
            width : 20,
            height : 80,
            velocity : 10,
            type_: ObstacleType::Cactus,
            last_time_update,
        }
    }

    /// Create a new rock obstacle
    pub fn new_rock(x: u16, y: u16, last_time_update : Instant) -> Self {
        Self {
            x,
            y,
            width : 20,
            height : 40,
            velocity : 10,
            type_: ObstacleType::Rock,
            last_time_update,
        }
    }

    /// Update the obstacle position
    pub fn update(&mut self, now: Instant) {
        let delta = now.duration_since(self.last_time_update).as_secs_f32();
        if delta < 0.0 {// if the time is in the future, we don't update
            return;
        }
        self.x -= (delta * self.velocity as f32) as u16;
        self.last_time_update = now;
    }
}


const GRAVITY : u8 = 10; // px/s^2


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dinosaur {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub velocity: u8,// vertical px/s
    pub last_time_update: Instant,// in seconds
}


impl Dinosaur {
    pub fn new_dinosaur(last_time_update : Instant) -> Self {
        Self {
            x : 50,
            y : 0,
            width : 50,
            height : 40,
            velocity : 0,
            last_time_update,
        }
    }

    /// jump
    fn intern_hump(&mut self, velocity : u8) -> bool {
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
        self.y += (self.velocity as f32 * delta) as u16;
        self.velocity -= (GRAVITY as f32 * delta) as u8;
        if self.y <= 0 {
            self.y = 0;
            self.velocity = 0;
        }
        self.last_time_update = tick;
    }
}