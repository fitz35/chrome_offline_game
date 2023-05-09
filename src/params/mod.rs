// ----------- basics ---------------------
pub const GAME_WIDTH : u16 = 500;
pub const GAME_HEIGHT : u16 = 300;

// ----------  game equilibrage -----------
// game physique
pub const GRAVITY : u8 = 10; // px/s^2

// game timing

/// increase the speed of the game every ... points
pub const SCORE_INCREASE_SPEED_INTERVAL : u8 = 20; 
// generation of new obstacle
pub const MIN_OBSTACLE_GENERATION_TIME : u8 = 1; // second
pub const MAX_OBSTACLE_GENERATION_TIME : u8 = 4; // second
pub const OBSTACLE_GENERATION_TIME_DECREASE_SPEED : u8 = 1; // second
// speed of obstacle
pub const MIN_OBSTACLE_SPEED : u8 = 10; // px/s
pub const MAX_OBSTACLE_SPEED : u8 = 30; // px/s
pub const OBSTACLE_INCREASE_SPEED_INTERVAL : u8 = 5; // px/s

// ---------- Entity ------------
pub const DINAUSOR_WIDTH : u16 = 20;
pub const DINAUSOR_HEIGHT : u16 = 50;

pub const CACTUS_WIDTH : u16 = 20;
pub const CACTUS_HEIGHT : u16 = 80;

pub const ROCK_WIDTH : u16 = 20;
pub const ROCK_HEIGHT : u16 = 20;