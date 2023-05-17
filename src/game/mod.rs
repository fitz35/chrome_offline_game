
use std::collections::HashSet;
use std::time::{Instant, Duration};

use rand::{SeedableRng, Rng};

use iced::widget::canvas::{Cursor, Geometry, Cache, Path, Stroke, LineCap, LineJoin, Frame, Text};
use iced::widget::{canvas, Canvas};
use iced::theme::{Theme};
use iced::{Application, executor, Command, Rectangle, Size, Color, Point, Subscription, keyboard};
use rand_pcg::Pcg64;

use crate::brain::Brain;
use crate::entity::{Dinosaur, Obstacle, ObstacleGenerateType, ObstacleEntityType};
use crate::neurone::{Neurone, NeuroneWebAction, get_color_from_neurone, NeuroneActivationCondition, NeuroneActivation, get_color_from_activation, get_color_from_action};
use crate::params::{GameParameters};
use crate::utils::{str_to_u8_array, get_scale_value, check_collision, remove_indexes};



pub struct Game {
    pub dinosaur: Dinosaur,
    pub obstacles: Vec<Obstacle>,
    pub score: u64,

    pub has_lost : bool,

    params : GameParameters,
    // ------ timing ------
    /// last time the game was updated
    pub last_time_update: Instant, 
    /// next time an obstacle will be generated
    pub next_obstacle_time: Instant, 
     /// time when the game started
    pub game_start_time: Instant,

    // ------ rng ------
    pub land_rng : Pcg64,

    // ------ auto play ------
    pub brain : Option<Brain>,

    // ------ display ------
    cache: Option<Cache>,
}

impl Game {
    pub fn new(params : &GameParameters, now : Instant, seed: &str, brain : Option<Brain>, cache : Option<Cache>) -> Self {
        let me = Self {
            dinosaur: Dinosaur::new_dinosaur(params, now),
            obstacles: Vec::new(),
            score: 0,
            has_lost : false,
            last_time_update : now,
            next_obstacle_time : now,
            game_start_time : now,
            land_rng : Pcg64::from_seed(str_to_u8_array(seed)),
            brain,
            cache,
            params : params.clone(),
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
    fn get_next_obstacle_timing(&self, last_time : Instant, score : u64) -> Instant {
        let interval = get_scale_value(
            self.params.max_obstacle_generation_time,
            self.params.min_obstacle_generation_time,
            self.params.obstacle_generation_time_decrease_speed,
            score,
            self.params.score_increase_speed_interval,
            true
        );
        
        last_time.checked_add(
            Duration::from_millis((interval * 1000.0) as u64)// apply the coef with reverse it
        ).unwrap()// WARN : error if the duration is too big
    }

    /// generate the next obstacle (add it to the vector) and update the timing
    fn generate_next_obstacle(&mut self) {
        let new_next_obstacle_time = self.get_next_obstacle_timing(
            self.next_obstacle_time, 
            self.score
        );
        let x = self.params.game_width as f64 + self.params.pterodactyle_offset_with_rock as f64;
        
        let random_obstacle_index = self.land_rng.gen_range(0..self.params.obstacle_generate_types.len());
        let random_obstacle: ObstacleGenerateType = self.params.obstacle_generate_types[random_obstacle_index].clone();

        match random_obstacle {
            ObstacleGenerateType::Cactus => {
                self.obstacles.push(Obstacle::new(
                    &self.params,
                    x, 
                    self.params.obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Cactus 
                ));
            },
            ObstacleGenerateType::Rock => {
                self.obstacles.push(Obstacle::new(
                    &self.params,
                    x, 
                    self.params.obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Rock
                ));
            },
            ObstacleGenerateType::RockAndPterodactyle => {
                self.obstacles.push(Obstacle::new(
                    &self.params,
                    x, 
                    self.params.obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Rock
                ));
                self.obstacles.push(Obstacle::new(
                    &self.params,
                    x, 
                    self.params.obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::PterodactyleWithRock
                ));
            },
            ObstacleGenerateType::RockAndHole => {
                self.obstacles.push(Obstacle::new(
                    &self.params,
                    x, 
                    self.params.obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Rock
                ));
                self.obstacles.push(Obstacle::new(
                    &self.params,
                    x, 
                    self.params.obstacle_speed, 
                    new_next_obstacle_time,
                    ObstacleEntityType::Hole
                ));
            },
            ObstacleGenerateType::Pterodactyle => {
                self.obstacles.push(Obstacle::new(
                    &self.params,
                    x, 
                    self.params.obstacle_speed, 
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

    // ............... display :
    /// symetric of the point with the height
    fn get_opposite(&self, point : Point, height : f32) -> Point {
        Point {
            x : point.x,
            y : (self.params.game_height as f32) - point.y - height
        }
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
    Brain(Brain, GameParameters),
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
                CustomFlags::None => {
                    let params = GameParameters::new_default();
                    Self::new(&params, Instant::now(), params.land_seed.as_str(), None, Some(Default::default()))
                }
                CustomFlags::Brain(brain, params) => Self::new(&params, Instant::now(), params.land_seed.as_str(), Some(brain), Some(Default::default())),
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
                    Duration::from_nanos(1000_000_000/self.params.game_fps as u64)
                ).unwrap();

                if !self.has_lost {
                    self.update(now);
                }
               
                // don't forget to clear the cache to force the redraw
                self.cache.as_ref().unwrap().clear();
                Command::none()
            },
            Message::Restart(brain) => {
                *self = Self::new(&GameParameters::new_default(), Instant::now(), self.params.land_seed.as_str(), brain, Some(Default::default()));
                Command::none()
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Canvas::new(self)
            .width(self.params.game_width)
            .height(self.params.game_height)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_nanos(1000_000_000/self.params.game_fps as u64)).map(|_| {
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
                    self.get_opposite(Point { x: (self.dinosaur.x as f32), y: (self.dinosaur.y as f32) }, self.dinosaur.height as f32), 
                    Size { width: (self.dinosaur.width as f32), height: (self.dinosaur.height as f32) }, 
                    Color::BLACK
                );

                // draw the obstacles
                for obstacle in self.obstacles.iter() {
                    frame.fill_rectangle(
                        self.get_opposite(Point { x: (obstacle.x as f32), y: (obstacle.y as f32) }, obstacle.height as f32), 
                        Size { width: (obstacle.width as f32), height: (obstacle.height as f32) }, 
                        Color::BLACK
                    );
                }

                // draw the brain
                match &self.brain {
                    Some(brain) => {
                        let action_activate = brain.get_activations(&self.obstacles);
                        draw_legend(frame, &action_activate);
                        for neurone_web in &brain.neurone_web {
                            let action = &neurone_web.action;
                            let mut last_neuron : Option<&Neurone> = None;
                            for neurone in &neurone_web.neurones {
                                let color_action = get_color_from_action(&action);
                                // draw highlight
                                let highlight_thickness = 2.0;
                                frame.fill_rectangle(
                                    self.get_opposite(Point { x: (neurone.x as f32), y: (neurone.y as f32) }, neurone.height as f32), 
                                    Size { width: (neurone.width as f32), height: (neurone.height as f32) }, 
                                    color_action
                                );

                                let width_without_thick = neurone.width as f32 - 2.0 * highlight_thickness;
                                let height_without_thick = neurone.height as f32 - 2.0 * highlight_thickness;
                                frame.fill_rectangle(
                                    self.get_opposite(
                                        Point { 
                                            x: (neurone.x as f32 + highlight_thickness), 
                                            y: (neurone.y as f32 + highlight_thickness) 
                                        }, 
                                        height_without_thick
                                    ), 
                                    Size { width: width_without_thick, height: height_without_thick }, 
                                    get_color_from_neurone(neurone)
                                );

                                // draw the link
                                match last_neuron {
                                    Some(last_neuron) => {
                                        let last_neuron_point = self.get_opposite(Point { 
                                            x: (last_neuron.x as f32) + (last_neuron.width as f32)/2.0, 
                                            y: (last_neuron.y as f32) - (last_neuron.height as f32)/2.0
                                        }, last_neuron.height as f32);
                                        let neurone_point = self.get_opposite(Point { 
                                            x: (neurone.x as f32) + (neurone.width as f32)/2.0, 
                                            y: (neurone.y as f32) - (neurone.height as f32)/2.0 
                                        }, last_neuron.height as f32);
                                        let path = Path::line(
                                            last_neuron_point, 
                                            neurone_point
                                        );
                                        frame.stroke(
                                            &path,
                                            Stroke {
                                                width: 1.0,
                                                line_cap: LineCap::Round,
                                                line_join: LineJoin::Round,
                                                ..Stroke::default()
                                                    .with_color(color_action)
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


fn draw_legend(frame : &mut Frame, action_activated :&HashSet<NeuroneWebAction>) {
    // draw all the possible activation of neurone :
    let all_conditions = vec![NeuroneActivationCondition::Air, NeuroneActivationCondition::Obstacle];
    let all_activations = vec![NeuroneActivation::Activate, NeuroneActivation::PreventActivate];
    let all_actions = vec![NeuroneWebAction::Jump, NeuroneWebAction::Bend, NeuroneWebAction::Unbend];


    let mut y = 0.0;
    for condition in all_conditions {
        for activation in &all_activations {
            let color = get_color_from_activation(*activation, condition);
            frame.fill_rectangle(
                Point { x: 0.0, y: y }, 
                Size { width: 20.0, height: 20.0 }, 
                color
            );
            frame.fill_text(Text {
                content: format!("{} when cross a {}", activation, condition),
                position: Point { x: 30.0, y: y },
                size: 20.0,
                color: Color::BLACK,
                ..Text::default()
            });

            y += 30.0;
        }
    }

    // action
    y = 0.0;
    let x: f32 = 350.0;
    let thickness = 2.0;
    for action in &all_actions {
        let color = get_color_from_action(action);
        frame.fill_rectangle(
            Point { x: x, y: y }, 
            Size { width: 20.0, height: 20.0 }, 
            color
        );

        // activate ?
        if !action_activated.contains(action) {
            frame.fill_rectangle(
                Point { x: x + thickness, y: y + thickness }, 
                Size { width: 20.0 - 2.0*thickness, height: 20.0 - 2.0*thickness }, 
                Color::WHITE
            );
        }

        frame.fill_text(Text {
            content: format!("{} action", action),
            position: Point { x: x + 30.0, y: y },
            size: 20.0,
            color: Color::BLACK,
            ..Text::default()
        });

        y += 30.0;
    }
}

// ----------------- tests -----------------

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_random_coherence() {
        // test the random number generator and the seed "test"
        let mut game = Game::new(&GameParameters::new_default(), Instant::now(), "test", None, None);
        let random_number0: u32 = game.land_rng.gen();
        let random_number1: u32 = game.land_rng.gen(); 
        let random_number2: u32 = game.land_rng.gen();
        let random_number3: u32 = game.land_rng.gen();
        let random_number4: u32 = game.land_rng.gen();

        assert_eq!(random_number0, 4210505251);
        assert_eq!(random_number1, 2381057059);
        assert_eq!(random_number2, 2142037166);
        assert_eq!(random_number3, 2036600936);
        assert_eq!(random_number4, 640383128);
    }
}