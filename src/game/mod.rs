use std::time::{Instant, Duration};
use iced::widget::canvas::{Cursor, Geometry, Cache};
use iced::widget::{canvas, Canvas};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use iced::theme::{self, Theme};
use iced::{Application, executor, Command, Rectangle, Size, Color, Point};

use crate::entity::{Dinosaur, Obstacle};
use crate::params;
use crate::utils::{str_to_u8_array, get_scale_value, check_collision};



pub struct Game {
    pub dinosaur: Dinosaur,
    pub obstacles: Vec<Obstacle>,
    pub score: u64,

    pub has_lost : bool,
    // ------ timing ------
    /// last time the game was updated
    pub last_time_update: Instant, 
    /// next time an obstacle will be generated
    pub next_obstacle_time: Instant, 
     /// time when the game started
    pub game_start_time: Instant,

    // ------ rng ------
    pub rng : ChaChaRng,

    // ------ display ------
    cache: Option<Cache>,
}

impl Game {
    pub fn new(now : Instant, seed: &str, cache : Option<Cache>) -> Self {
        let mut me = Self {
            dinosaur: Dinosaur::new_dinosaur(now),
            obstacles: Vec::new(),
            score: 0,
            has_lost : false,
            last_time_update : now,
            next_obstacle_time : now,
            game_start_time : now,
            rng : ChaChaRng::from_seed(str_to_u8_array(seed)),
            cache,
        };
        me.update(now);
        me
    }
    // ---------------- game state ----------------

    pub fn update(&mut self, now: Instant) {
        self.dinosaur.update(now);

        // update all obstacle
        self.update_all_obstacles(now);

        // check the hitbox
        for obstacle in self.obstacles.iter() {
            if check_collision(
                self.dinosaur.x,
                self.dinosaur.y,
                self.dinosaur.width,
                self.dinosaur.height,

                obstacle.x,
                obstacle.y,
                obstacle.width,
                obstacle.height
            ){
                self.has_lost = true;
                return;
            }
        }

        // calculate the next obstacle if we have passed the time
        if now >= self.next_obstacle_time {
            self.generate_next_obstacle();
        }
    }

    pub fn jump(&mut self) {
        self.dinosaur.jump();
    }

    // ---------------- helper and game generation ---------

    // ................. obstacle : 

    /// get the timing for the next obstacle (accelerate 4 times)
    fn get_next_obstacle_timing(last_time : Instant, score : u64) -> Instant {
        let interval = get_scale_value(
            params::MAX_OBSTACLE_GENERATION_TIME,
            params::MIN_OBSTACLE_GENERATION_TIME,
            params::OBSTACLE_GENERATION_TIME_DECREASE_SPEED,
            score,
            params::SCORE_INCREASE_SPEED_INTERVAL,
            true
        );
        
        last_time.checked_add(
            Duration::from_secs(interval as u64)// apply the coef with reverse it
        ).unwrap()// WARN : error if the duration is too big
    }

    /// generate a random (given the random generator) new obstacle with speed adapted to the score
    /// at the time time_to_appear
    fn generate_next_obstacle_entity(x : i16, time_to_appear : Instant, score : u64, rng : &mut ChaChaRng) -> Obstacle {
        let velocity = get_scale_value(
            params::MAX_OBSTACLE_SPEED,
            params::MIN_OBSTACLE_SPEED,
            params::OBSTACLE_INCREASE_SPEED_INTERVAL,
            score,
            params::SCORE_INCREASE_SPEED_INTERVAL,
            false
        );
        Obstacle::new_random(x, velocity, time_to_appear, rng)
    }

    /// generate the next obstacle (add it to the vector) and update the timing
    fn generate_next_obstacle(&mut self) {
        let new_next_obstacle_time = Self::get_next_obstacle_timing(
            self.next_obstacle_time, 
            self.score
        );
        let next_obstacle = Self::generate_next_obstacle_entity(
            params::GAME_WIDTH as i16, 
            new_next_obstacle_time, 
            self.score,
            &mut self.rng
        );
        // update timing
        self.next_obstacle_time = new_next_obstacle_time;
        // update the vector
        self.obstacles.push(next_obstacle);
    }

    /// update all the obstacle and remove (add 1 to the score) the obstacle outside the screen
    fn update_all_obstacles(&mut self, now : Instant) {
        let mut to_remove : Vec<usize> = Vec::new();
        let mut i : usize = 0;
        for obstacle in self.obstacles.iter_mut() {
            obstacle.update(now);

            if (obstacle.x + obstacle.width as i16) < 0 {
                to_remove.push(i);
            }
            i = i + 1;
        }

        self.score += to_remove.len() as u64;
        for index_to_remove in to_remove.iter() {
            self.obstacles.remove(*index_to_remove);
        }
    }
}

// ----------------- front -----------------
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Jump(),
    Update(Instant),
}

// define the application for iced on the game
impl Application for Game {
    type Theme = Theme;
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            // construct the game at the beginning
            Self::new(Instant::now(), params::LAND_SEED, Some(Default::default())),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chrome Dinosaur")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Jump() => {
                self.jump();
                Command::none()
            },
            Message::Update(now) => {
                self.update(now);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Canvas::new(self)
            .width(300)
            .height(300)
            .into()
    }
}


impl<Message> canvas::Program<Message> for Game {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.as_ref().unwrap().draw(bounds.size(), |frame| {
            frame.fill_rectangle(
                Point { x: (0.0), y: (0.0) }, 
                Size { width: (100.0), height: (100.0) }, 
                Color::BLACK
            )
        });

        vec![geometry]
    }
}

// ----------------- tests -----------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_coherence() {
        // test the random number generator and the seed "test"
        let mut game = Game::new(Instant::now(), "test", None);
        let random_number1: u32 = game.rng.gen(); 
        let random_number2: u32 = game.rng.gen();
        let random_number3: u32 = game.rng.gen();
        let random_number4: u32 = game.rng.gen();

        assert_eq!(random_number1, 1389975915);
        assert_eq!(random_number2, 2957384555);
        assert_eq!(random_number3, 2883209199);
        assert_eq!(random_number4, 3610659652);
    }
}