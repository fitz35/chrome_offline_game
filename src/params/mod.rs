// ----------- basics ---------------------
pub const GAME_WIDTH : u16 = 1280;
pub const GAME_HEIGHT : u16 = 720;

pub const GAME_FPS : u8 = 60;
// ----------- display ---------------------

// ----------  game equilibrage -----------
pub const LAND_SEED: &str = "42";
// game physique
pub const GRAVITY : u64 = 2000; // px/s^2


pub const DINAUSOR_JUMP_VELOCITY : f64 = 800.0; // px/s
// game timing

/// increase the speed of the game every ... points
pub const SCORE_INCREASE_SPEED_INTERVAL : f64 = 2.0; 
// generation of new obstacle
pub const MIN_OBSTACLE_GENERATION_TIME : f64 = 1.2; // second
pub const MAX_OBSTACLE_GENERATION_TIME : f64 = 2.0; // second
pub const OBSTACLE_GENERATION_TIME_DECREASE_SPEED : f64 = 0.2; // second
// speed of obstacle
pub const OBSTACLE_SPEED : f64 = 400.0; // px/s

// ---------- Entity ------------
pub const DINAUSOR_WIDTH : u16 = 40;
pub const DINAUSOR_HEIGHT : u16 = 100;

pub const CACTUS_WIDTH : u16 = 40;
pub const CACTUS_HEIGHT : u16 = 80;

pub const ROCK_WIDTH : u16 = 40;
pub const ROCK_HEIGHT : u16 = 40;

pub const PTERODACTYLE_WIDTH : u16 = 40;
pub const PTERODACTYLE_HEIGHT : u16 = 40;
pub const PTERODACTYLE_FLYING_HEIGHT : u16 = 110;
pub const PTERODACTYLE_OFFSET : u16 = 130;