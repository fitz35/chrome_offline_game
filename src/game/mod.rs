use std::time::{Instant, Duration};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::entity::{Dinosaur, Obstacle};
use crate::utils::str_to_u8_array;



pub struct Game {
    pub dinosaur: Dinosaur,
    pub obstacles: Vec<Obstacle>,
    pub score: u64,
    // ------ timing ------
    /// last time the game was updated
    pub last_time_update: Instant, 
    /// next time an obstacle will be generated
    pub next_obstacle_time: Instant, 
     /// time when the game started
    pub game_start_time: Instant,

    pub rng : ChaChaRng,
}

impl Game {
    pub fn new(now : Instant, seed: &str) -> Self {
        let mut me = Self {
            dinosaur: Dinosaur::new_dinosaur(now),
            obstacles: Vec::new(),
            score: 0,
            last_time_update : now,
            next_obstacle_time : Self::get_next_obstacle_timing(now, 0),
            game_start_time : now,
            rng : ChaChaRng::from_seed(str_to_u8_array(seed))
        };
        me.update(now);
        me
    }
    // ---------------- game state ----------------

    pub fn update(&mut self, tick: Instant) {
        self.dinosaur.update(tick);
        
    }

    pub fn jump(&mut self) {
        self.dinosaur.jump();
    }

    pub fn long_jump(&mut self) {
        self.dinosaur.long_jump();
    }

    // ---------------- helper and game generation ---------

    /// get the timing for the next obstacle (accelerate 4 times)
    fn get_next_obstacle_timing(last_time : Instant, score : u64) -> Instant {
        let max_coef = 4;
        let base_timing = 1; // second
        let coef = score / 20 ;// u64
        let keep_coef = max_coef.min(coef);
        
        last_time.checked_add(
            Duration::from_secs(base_timing * (max_coef - keep_coef + 1))// apply the coef with reverse it
        ).unwrap()// WARN : error if the duration is too big
    }
}