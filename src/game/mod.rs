use std::time::{Instant, Duration};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use iced::widget::canvas::{Cursor, Geometry, Cache};
use iced::widget::{canvas, Canvas};
use iced::theme::{Theme};
use iced::{Application, executor, Command, Rectangle, Size, Color, Point, Subscription, window, keyboard};

use crate::entity::{Dinosaur, Obstacle, ObstacleGenerateType};
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
            Duration::from_millis((interval * 1000.0) as u64)// apply the coef with reverse it
        ).unwrap()// WARN : error if the duration is too big
    }

    /// generate the next obstacle (add it to the vector) and update the timing
    fn generate_next_obstacle(&mut self) {
        let new_next_obstacle_time = Self::get_next_obstacle_timing(
            self.next_obstacle_time, 
            self.score
        );
        let x = params::GAME_WIDTH as f64 + params::PTERODACTYLE_OFFSET as f64;
        
        let random_obstacle: ObstacleGenerateType = match self.rng.gen_range(0..3) {
            0 => ObstacleGenerateType::Cactus,
            1 => ObstacleGenerateType::Rock,
            2 => ObstacleGenerateType::RockAndPterodactyle,
            _ => panic!("impossible"),
        };

        match random_obstacle {
            ObstacleGenerateType::Cactus => {
                self.obstacles.push(Obstacle::new_cactus(x, params::OBSTACLE_SPEED, new_next_obstacle_time));
            },
            ObstacleGenerateType::Rock => {
                self.obstacles.push(Obstacle::new_rock(x, params::OBSTACLE_SPEED, new_next_obstacle_time));
            },
            ObstacleGenerateType::RockAndPterodactyle => {
                self.obstacles.push(Obstacle::new_rock(x, params::OBSTACLE_SPEED, new_next_obstacle_time));
                self.obstacles.push(Obstacle::new_pterodactyle(
                    x, 
                    params::OBSTACLE_SPEED, 
                    new_next_obstacle_time
                ));
            },
        }

        self.next_obstacle_time = new_next_obstacle_time;
    }

    /// update all the obstacle and remove (add 1 to the score) the obstacle outside the screen
    fn update_all_obstacles(&mut self, now : Instant) {
        
        let mut to_remove : Vec<usize> = Vec::new();
        for (i, obstacle) in self.obstacles.iter_mut().enumerate() {
            obstacle.update(now);

            if (obstacle.x + obstacle.width as f64) < 0.0 {
                to_remove.push(i);
            }
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
    Jump,
    Update(Instant),
}

// define the application for iced on the game
impl Application for Game {
    type Theme = Theme;
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            // construct the game at the beginning
            Self::new(Instant::now(), params::LAND_SEED, Some(Default::default())),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chrome Dinosaur")
    }

    // handle the message
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Jump => {
                self.jump();
                Command::none()
            },
            Message::Update(now) => {
                self.update(now);
                // don't forget to clear the cache to force the redraw
                self.cache.as_ref().unwrap().clear();
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Canvas::new(self)
            .width(params::GAME_WIDTH)
            .height(params::GAME_HEIGHT)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Update)
    }
}


impl canvas::Program<Message> for Game {
    type State = ();

    // catch the event 
    fn update(
            &self,
            _state: &mut Self::State,
            event: canvas::Event,
            _bounds: Rectangle,
            _cursor: Cursor,
        ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Keyboard(keyboard_event) => {
                match keyboard_event {
                   keyboard::Event::CharacterReceived(' ') => (canvas::event::Status::Captured, Some(Message::Jump)),
                    _ => (canvas::event::Status::Ignored, None)
                }
            },
            _ => (canvas::event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        // dont forget the as-ref (option) and the unwrap (can throw erreur if the cache is not initialized)
        let geometry = self.cache.as_ref().unwrap().draw(bounds.size(), |frame| {
            
            // draw the dinosaur
            frame.fill_rectangle(
                Point { x: (self.dinosaur.x as f32), y: (self.dinosaur.y as f32) }, 
                Size { width: (self.dinosaur.width as f32), height: (self.dinosaur.height as f32) }, 
                Color::BLACK
            );

            // draw the obstacles
            for obstacle in self.obstacles.iter() {
                frame.fill_rectangle(
                    Point { x: (obstacle.x as f32), y: (obstacle.y as f32) }, 
                    Size { width: (obstacle.width as f32), height: (obstacle.height as f32) }, 
                    Color::BLACK
                )
            }
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