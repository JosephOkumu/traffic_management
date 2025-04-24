use std::rc::Rc;
use rand::Rng;

use sdl2::{pixels::Color, rect::{Point, Rect}};
use crate::entities::*;
use crate::map::TrafficLight;
use sdl2::render::Texture;


const BASE_VELOCITY: u32 = 4;
const SLOW_VELOCITY: u32 = 2;  // Reduced speed for safety
const SAFE_DISTANCE: u32 = 40; // Increased safe distance
const TURN_VELOCITY: u32 = 2;  // Slower speed for turns
const DETECTION_OFFSET: i32 = 10; // Increased detection offset

const TRECTS: [(&str,i32,i32,u32,u32);12] = 
[
    ("bmw",179,89,25,47),
    ("raptor",290,199,28,55),
    ("landcruiser",1,1,29,56),
    ("landcruiser2",197,198,29,56),
    ("raptor2",320,199,28,55),
    ("suv",380,204,28,50),
    ("suv2",410,204,28,50),
    ("mustang2",66,80,26,49),
    ("camaro",87,134,26,48),
    ("camaro2",94,84,26,48),
    ("challenger2",115,135,28,48),
    ("challenger3",145,138,28,48)
];

// #[derive(Debug,Clone)]
pub enum DisplayType<'a> {
    Texture(Rc<&'a Texture<'a>>),
    Color(Color),
}

impl Clone for DisplayType<'_> {
    fn clone(&self) -> Self {
       match *self {
           DisplayType::Texture(ref t) => DisplayType::Texture(Rc::clone(t)),
           DisplayType::Color(c) => DisplayType::Color(c),
       }
    }
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum UpdateState {
    Moving,
    Slowing,

    Finished,
    Collided,
    Waiting,
}

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone)]
pub struct Car<'a> {
    // Boxs
    hit_box: Rect,
    detection_lower: Rect,
    detection_upper: Rect,

    // Visual
    sprite: DisplayType<'a>,
    t_rect: Option<Rect>,

    // Logic
    state: UpdateState,
    velocity: u32,
    w_l: (u32,u32),
    path: Vec<Point>,
    current_direction: Direction,

    debug: bool,
    current_light: Option<Point>, // Track the first traffic light encountered
}

impl<'a> From<&'a Texture<'a>> for DisplayType<'a> {
    fn from(texture: &'a Texture<'a>) -> Self {
        DisplayType::Texture(Rc::new(texture))
    }
}

impl<'a> From<Color> for DisplayType<'a> {
    fn from(c: Color) -> Self {
        DisplayType::Color(c)
    }
}



impl Direction {
    pub fn to_angle(&self) -> f64 {
        match self {
            Direction::East => 90.0,
            Direction::South => 0.0,
            Direction::West => -90.0,
            Direction::North => 180.0,
        }
    }
}


impl<'a> Car<'a> {
    pub fn new<T: Into<DisplayType<'a>>>(center: Point, w: u32, l: u32, sprite: T) -> Self {
        let hit_box = Rect::from_center(center, w, l);
        Self {
            hit_box: hit_box,
            w_l: (w,l),
            sprite: sprite.into(),
            velocity: BASE_VELOCITY,
            state: UpdateState::Moving,
            path: Vec::new(),
            detection_lower: hit_box,
            detection_upper: hit_box,
            current_direction: Direction::North,
            debug: false,
            t_rect: None,
            current_light: None,
        }
    }

    pub fn set_path(&mut self,path: Vec<Point>) {
        self.path = path;
    }

    pub fn set_velocity(&mut self,velocity: u32) {
        self.velocity = velocity;
    }

    pub fn set_debug(&mut self,b: bool) {
        self.debug = b;
    }

    fn is_on_right(&self,other: &Self) -> bool{
        match (self.current_direction,other.current_direction) {
            (Direction::South,Direction::East) => true,
            (Direction::East,Direction::North) => true,
            (Direction::North,Direction::West) => true,
            (Direction::West,Direction::South) => true,
            _ => false,
        }
    }

    pub fn get_detections(&self) -> Option<(Rect,Rect)>{
        Some((self.detection_lower,self.detection_upper))  
    }

    pub fn set_texture(&mut self,texture: &'a Texture<'a>) {
        self.sprite = DisplayType::from(texture);
        let i: usize = rand::thread_rng().gen_range(0..TRECTS.len());
        let v = TRECTS[i];
        self.t_rect = Some(Rect::new(v.1, v.2, v.3, v.4))
    }

    pub fn update(&mut self, others: Vec<Car>, traffic_lights: &[TrafficLight]) -> UpdateState {
        if self.path.is_empty() {
            self.state = UpdateState::Finished;
            return UpdateState::Finished; // No more points to reach
        }
    
        // Check traffic lights only if we haven't entered the intersection
        if !self.has_entered_intersection() {
            let light_detection = Rect::from_center(self.hit_box.center(), 100, 100);
            
            // Define the coordinates for left-side traffic lights based on car's direction
            let left_light_pos = match self.current_direction {
                Direction::North => Point::new(470, 470), // Left light for northbound
                Direction::South => Point::new(610, 610), // Left light for southbound
                Direction::East => Point::new(470, 610),  // Left light for eastbound
                Direction::West => Point::new(610, 470),  // Left light for westbound
            };
            
            match self.current_light {
                // If we already have a tracked light, check only that one
                Some(tracked_position) => {
                    if tracked_position == left_light_pos {
                        for light in traffic_lights {
                            if light.position == tracked_position {
                                if !light.is_green() {
                                    self.state = UpdateState::Waiting;
                                    return UpdateState::Waiting;
                                }
                                break;
                            }
                        }
                    }
                },
                // If we don't have a tracked light yet, only consider the left light
                None => {
                    for light in traffic_lights {
                        if light.position == left_light_pos &&
                           light_detection.has_intersection(Rect::from_center(light.position, 100, 100)) {
                            self.current_light = Some(light.position);
                            if !light.is_green() {
                                self.state = UpdateState::Waiting;
                                return UpdateState::Waiting;
                            }
                            break;
                        }
                    }
                }
            }
        } else {
            // Reset light tracking once we've entered the intersection
            self.current_light = None;
        }

        let target = self.path[0]; // Prochain point à atteindre
        let position = self.hit_box.center();
    
        let direction = (
            (target.x - position.x) as f32,
            (target.y - position.y) as f32,
        );

        let card_direction = match (direction.0.abs() > direction.1.abs(),direction.0 <= 0.0,direction.1 <= 0.0) {
            (true,false,_) => Direction::East,
            (true,true,_) => Direction::West,
            (false,_,true) => Direction::South,
            (false,_,false) => Direction::North,
        };

        self.current_direction = card_direction;


        let distance = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
    
        if distance < self.velocity as f32 {
            self.hit_box = Rect::from_center(target, self.hit_box.width(), self.hit_box.height());
            self.path.remove(0);
        } else {
            let normalized = (direction.0 / distance, direction.1 / distance);
            let movement = (
                (normalized.0 * self.velocity as f32) as i32,
                (normalized.1 * self.velocity as f32) as i32,
            );

            let new_hitbox = if card_direction == Direction::North || card_direction == Direction::South {
                Rect::from_center(
                    Point::new(position.x + movement.0, position.y + movement.1),
                    self.w_l.0,
                    self.w_l.1,
                )
            } else {
                Rect::from_center(
                    Point::new(position.x + movement.0, position.y + movement.1),
                    self.w_l.1,
                    self.w_l.0,
                )
            };
    

            let angle = direction.1.atan2(direction.0); 
    
            let perpendicular = (-angle.sin(), angle.cos()); 
            let inc = if card_direction == Direction::North || card_direction == Direction::South {
                new_hitbox.height()
            } else {
                new_hitbox.width()
            };
            
            let detection_distance = inc as f32 * 1.4; 
            let detection_x = position.x as f32 + detection_distance * angle.cos()
                + DETECTION_OFFSET as f32 * perpendicular.0;
            let detection_y = position.y as f32 + detection_distance * angle.sin()
                + DETECTION_OFFSET as f32 * perpendicular.1;
            

            let ahead_box_lower = Rect::from_center(
                Point::new(detection_x as i32, detection_y as i32),
                self.hit_box.width() + SAFE_DISTANCE,
                self.hit_box.height() + SAFE_DISTANCE,
            );
            
            let detection_distance = inc as f32 * 2.8; 
            let detection_x = position.x as f32 + detection_distance * angle.cos()
                + DETECTION_OFFSET as f32 * perpendicular.0;
            let detection_y = position.y as f32 + detection_distance * angle.sin()
                + DETECTION_OFFSET as f32 * perpendicular.1;

            let ahead_box_upper = Rect::from_center(
                Point::new(detection_x as i32, detection_y as i32),
                self.hit_box.width() + SAFE_DISTANCE,
                self.hit_box.height() + SAFE_DISTANCE,
            );
            self.detection_lower = ahead_box_lower;
            self.detection_upper = ahead_box_upper;
            // Check for collisions and maintain safe distance
            for other in &others {
                if self.hit_box.has_intersection(other.hit_box) {
                    self.state = UpdateState::Collided;
                    return UpdateState::Collided;
                }

                // Early detection for slowing down - more aggressive
                if (ahead_box_upper.has_intersection(other.detection_upper) ||
                    ahead_box_upper.has_intersection(other.detection_lower)) &&
                    other.state != UpdateState::Slowing {
                    self.state = UpdateState::Slowing;
                    self.velocity = SLOW_VELOCITY;
                    self.hit_box = new_hitbox;
                    return UpdateState::Slowing;
                }
                
                // Stop and wait if too close to other vehicles
                if !self.has_entered_intersection() {
                    let is_too_close = ahead_box_lower.has_intersection(other.get_hitbox()) ||
                                     ahead_box_upper.has_intersection(other.detection_lower) ||
                                     ahead_box_upper.has_intersection(other.get_hitbox());
                    
                    if is_too_close {
                        // Allow passing only if other car is waiting and we have right of way
                        if other.state == UpdateState::Waiting &&
                           self.is_on_right(other) &&
                           !ahead_box_lower.has_intersection(other.get_hitbox()) {
                            // Proceed with caution
                            self.velocity = SLOW_VELOCITY;
                            continue;
                        }
                        // Otherwise wait
                        self.state = UpdateState::Waiting;
                        return UpdateState::Waiting;
                    }
                }

                // Adjust speed for turns
                if self.has_entered_intersection() {
                    self.velocity = TURN_VELOCITY;
                }
                
            }
    
            self.hit_box = new_hitbox; 
        };
        self.velocity = BASE_VELOCITY;

        self.state = UpdateState::Moving;
        return UpdateState::Moving;
    }

    fn has_entered_intersection(&self) -> bool {
        let intersection_area = Rect::new(470, 470, 140, 140); // Define the intersection bounds
        self.hit_box.has_intersection(intersection_area)
    }

}


impl<'a> Entity for Car<'a> {
    fn display(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), Box<dyn std::error::Error>> {
        if let DisplayType::Color(c) = &self.sprite {
            canvas.set_draw_color(*c);
            canvas.fill_rect(self.get_hitbox())?;
            return Ok(());
        }
        if let DisplayType::Texture(texture) = &self.sprite {
            let angle = self.current_direction.to_angle();
            if self.debug {
                canvas.set_draw_color(Color::GREEN);
                canvas.draw_rect(self.hit_box)?;
            }
            canvas.copy_ex(
                texture, 
                self.t_rect, 
                Some(Rect::from_center(self.hit_box.center(),self.w_l.0,self.w_l.1)), 
                angle, 
                None, 
                false, 
                false
            )?;
            return Ok(());
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Texture is not implemented yet")))
    }

    fn get_hitbox(&self) -> Rect {
        self.hit_box
    }
}