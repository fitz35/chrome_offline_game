
use std::collections::HashSet;
use std::time::{Instant, Duration};

use rand::{Rng, SeedableRng};

use iced::widget::canvas::{Cursor, Geometry, Cache, Path, Stroke, LineCap, LineJoin};
use iced::widget::{canvas, Canvas};
use iced::theme::{Theme};
use iced::{Application, executor, Command, Rectangle, Size, Color, Point, Subscription, keyboard};
use rand_pcg::Pcg64;

use crate::brain::Brain;
use crate::entity::{Dinosaur, Obstacle, ObstacleGenerateType, ObstacleEntityType};
use crate::neurone::{Neurone, NeuroneWebAction};
use crate::params::{PARAMS};
use crate::utils::{str_to_u8_array, get_scale_value, check_collision, remove_indexes};



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
    pub rng : Pcg64,

    // ------ auto play ------
    pub brain : Option<Brain>,

    // ------ display ------
    cache: Option<Cache>,
}

impl Game {
    pub fn new(now : Instant, seed: &str, brain : Option<Brain>, cache : Option<Cache>) -> Self {
        let me = Self {
            dinosaur: Dinosaur::new_dinosaur(now),
            obstacles: Vec::new(),
            score: 0,
            has_lost : false,
            last_time_update : now,
            next_obstacle_time : now,
            game_start_time : now,
            rng : Pcg64::from_seed(str_to_u8_array(seed)),
            brain,
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

        // if we have a brain, we use it
        self.do_actions(self.get_brain_actions());
        self.last_time_update = now;
    }

    // -----------------    actions    -----------------
    /// do one action
    fn do_action(&mut self, action : &NeuroneWebAction) {
        match action {
            NeuroneWebAction::Jump => {
                self.dinosaur.jump();
            },
            NeuroneWebAction::Bend => {
                self.dinosaur.bend();
            },
            NeuroneWebAction::Unbend => {
                self.dinosaur.unbend();
            },
        }
    }

    /// doall the actions in the set
    pub fn do_actions(&mut self, actions : HashSet<NeuroneWebAction>) {
        for action in actions {
            self.do_action(&action);
        }
    }

    /// do all the actions of the brain (if there is one)
    fn get_brain_actions(&self) -> HashSet<NeuroneWebAction> {
        if self.brain.is_none() {
            HashSet::new()
        }else{
            self.brain.as_ref().unwrap().get_activations(&self.obstacles)
        }
    }

    // ---------------- helper and game generation ---------

    // ................. obstacle : 

    /// get the timing for the next obstacle (accelerate 4 times)
    fn get_next_obstacle_timing(last_time : Instant, score : u64) -> Instant {
        let interval = get_scale_value(
            (*PARAMS).max_obstacle_generation_time,
            (*PARAMS).min_obstacle_generation_time,
            (*PARAMS).obstacle_generation_time_decrease_speed,
            score,
            (*PARAMS).score_increase_speed_interval,
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
        let x = (*PARAMS).game_width as f64 + (*PARAMS).pterodactyle_offset_with_rock as f64;
        
        let random_obstacle: ObstacleGenerateType = match self.rng.gen_range(0..5) {
            0 => ObstacleGenerateType::Cactus,
            1 => ObstacleGenerateType::Rock,
            2 => ObstacleGenerateType::RockAndPterodactyle,
            3 => ObstacleGenerateType::RockAndHole,
            4 => ObstacleGenerateType::Pterodactyle,
            _ => panic!("impossible"),
        };

        match random_obstacle {
            ObstacleGenerateType::Cactus => {
                self.obstacles.push(Obstacle::new(
                    x, 
                    (*PARAMS).obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Cactus 
                ));
            },
            ObstacleGenerateType::Rock => {
                self.obstacles.push(Obstacle::new(
                    x, 
                    (*PARAMS).obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Rock
                ));
            },
            ObstacleGenerateType::RockAndPterodactyle => {
                self.obstacles.push(Obstacle::new(
                    x, 
                    (*PARAMS).obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Rock
                ));
                self.obstacles.push(Obstacle::new(
                    x, 
                    (*PARAMS).obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::PterodactyleWithRock
                ));
            },
            ObstacleGenerateType::RockAndHole => {
                self.obstacles.push(Obstacle::new(
                    x, 
                    (*PARAMS).obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Rock
                ));
                self.obstacles.push(Obstacle::new(
                    x, 
                    (*PARAMS).obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Hole
                ));
            },
            ObstacleGenerateType::Pterodactyle => {
                self.obstacles.push(Obstacle::new(
                    x, 
                    (*PARAMS).obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Pterodactyle
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
        remove_indexes(&mut self.obstacles, &to_remove);
    }
}

// ----------------- front -----------------
#[derive(Debug, Clone)]
pub enum Message {
    Do(NeuroneWebAction),
    Restart(Option<Brain>),
    Update,
}

#[derive(Debug, Clone)]
pub enum CustomFlags {
    None,
    Brain(Brain),
}

// define the default value for the flags
impl Default for CustomFlags {
    fn default() -> Self {
        // Define the default values for CustomFlags here
        CustomFlags::None
    }
}

// define the application for iced on the game
impl Application for Game {
    type Theme = Theme;
    type Executor = executor::Default;
    type Message = Message;
    type Flags = CustomFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            // construct the game at the beginning
            match flags {
                CustomFlags::None => Self::new(Instant::now(), (*PARAMS).land_seed.as_str(), None, Some(Default::default())),
                CustomFlags::Brain(brain) => Self::new(Instant::now(), (*PARAMS).land_seed.as_str(), Some(brain), Some(Default::default())),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chrome Dinosaur")
    }

    // handle the message
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Do(action) => {
                if !self.has_lost {
                    self.do_action(&action);
                }
                Command::none()
            },
            Message::Update => {
                let now = self.last_time_update.checked_add(
                    Duration::from_nanos(1000_000_000/(*PARAMS).game_fps as u64)
                ).unwrap();

                if !self.has_lost {
                    self.update(now);
                }
               
                // don't forget to clear the cache to force the redraw
                self.cache.as_ref().unwrap().clear();
                Command::none()
            },
            Message::Restart(brain) => {
                *self = Self::new(Instant::now(), (*PARAMS).land_seed.as_str(), brain, Some(Default::default()));
                Command::none()
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Canvas::new(self)
            .width((*PARAMS).game_width)
            .height((*PARAMS).game_height)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_nanos(1000_000_000/(*PARAMS).game_fps as u64)).map(|_| {
            Message::Update
        })
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
                    keyboard::Event::CharacterReceived(' ') => {
                        if self.has_lost {
                            // restart the game
                            return (canvas::event::Status::Captured, Some(Message::Restart(self.brain.clone())))
                        }
                        // unbend if bending
                        if self.dinosaur.is_bending {
                            return (canvas::event::Status::Captured, Some(Message::Do(NeuroneWebAction::Unbend)))
                        }

                        // jump
                        (canvas::event::Status::Captured, Some(Message::Do(NeuroneWebAction::Jump)))
                    },
                    keyboard::Event::CharacterReceived('s') => {
                        (canvas::event::Status::Captured, Some(Message::Do(NeuroneWebAction::Bend)))
                    },
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
            if self.has_lost {
                frame.fill_text(format!("Lost (press space to restart): {}", self.score));
            }else{
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
                    );
                }

                // draw the brain
                match &self.brain {
                    Some(brain) => {
                        for neurone_web in &brain.neurone_web {
                            let action = &neurone_web.action;
                            let mut last_neuron : Option<&Neurone> = None;
                            for neurone in &neurone_web.neurones {
                                frame.fill_rectangle(
                                    Point { x: (neurone.x as f32), y: (neurone.y as f32) }, 
                                    Size { width: (neurone.width as f32), height: (neurone.height as f32) }, 
                                    neurone.get_color()
                                );

                                // draw the link
                                match last_neuron {
                                    Some(last_neuron) => {
                                        let last_neuron_point = Point { 
                                            x: (last_neuron.x as f32) + (last_neuron.width as f32)/2.0, 
                                            y: (last_neuron.y as f32) + (last_neuron.height as f32)/2.0
                                        };
                                        let neurone_point = Point { 
                                            x: (neurone.x as f32) + (neurone.width as f32)/2.0, 
                                            y: (neurone.y as f32) + (neurone.height as f32)/2.0 
                                        };
                                        let path = Path::line(
                                            last_neuron_point, 
                                            neurone_point
                                        );
                                        let color = match action {
                                            NeuroneWebAction::Jump => Color::from_rgb8(0, 255, 0),
                                            NeuroneWebAction::Bend => Color::from_rgb8(255, 0, 0),
                                            NeuroneWebAction::Unbend => Color::from_rgb8(0, 0, 255),
                                        };
                                        frame.stroke(
                                            &path,
                                            Stroke {
                                                width: 1.0,
                                                line_cap: LineCap::Round,
                                                line_join: LineJoin::Round,
                                                ..Stroke::default()
                                                    .with_color(color)
                                                    .clone()
                                            }
                                            
                                        );
                                    },
                                    None => {}
                                }
                                last_neuron = Some(neurone);
                            }
                        }
                    },
                    None => {}
                }
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
        let mut game = Game::new(Instant::now(), "test", None, None);
        let random_number0: u32 = game.rng.gen();
        let random_number1: u32 = game.rng.gen(); 
        let random_number2: u32 = game.rng.gen();
        let random_number3: u32 = game.rng.gen();
        let random_number4: u32 = game.rng.gen();

        assert_eq!(random_number0, 4210505251);
        assert_eq!(random_number1, 2381057059);
        assert_eq!(random_number2, 2142037166);
        assert_eq!(random_number3, 2036600936);
        assert_eq!(random_number4, 640383128);
    }
}