#![allow(dead_code)]

use sdl2::{pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window};
use rand::Rng;
use crate::cars::Car;

const BORDER_UP_LEFT: i32 = -40;
const BORDER_DOWN_RIGHT: i32 = 1120;

// W : LEFT
// E : RIGHT

// Done
// Path coordinates aligned with the two-lane roads
// Northbound lane (left side of vertical road)
const N_S: [(i32,i32); 3] = [(490,BORDER_UP_LEFT),(490,490),(490,BORDER_DOWN_RIGHT)];
const N_E: [(i32,i32); 3] = [(490,BORDER_UP_LEFT),(490,540),(BORDER_DOWN_RIGHT,590)];
const N_W: [(i32,i32); 3] = [(490,BORDER_UP_LEFT),(490,490),(BORDER_UP_LEFT,490)];

// Southbound lane (right side of vertical road)
const S_N: [(i32,i32); 3] = [(590,BORDER_DOWN_RIGHT),(590,590),(590,BORDER_UP_LEFT)];
const S_E: [(i32,i32); 3] = [(590,BORDER_DOWN_RIGHT),(590,590),(BORDER_DOWN_RIGHT,590)];
const S_W: [(i32,i32); 3] = [(590,BORDER_DOWN_RIGHT),(590,490),(BORDER_UP_LEFT,490)];

// Westbound lane (top side of horizontal road)
const E_W: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,490),(490,490),(BORDER_UP_LEFT,490)];
const E_N: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,490),(590,490),(590,BORDER_UP_LEFT)];
const E_S: [(i32,i32); 3] = [(BORDER_DOWN_RIGHT,490),(540,490),(540,BORDER_DOWN_RIGHT)];

// Eastbound lane (bottom side of horizontal road)
// Eastbound paths - adjusted to maintain lane consistency
const W_E: [(i32,i32); 3] = [(BORDER_UP_LEFT,590),(590,590),(BORDER_DOWN_RIGHT,590)];
const W_N: [(i32,i32); 3] = [(BORDER_UP_LEFT,590),(540,590),(490,BORDER_UP_LEFT)];
const W_S: [(i32,i32); 3] = [(BORDER_UP_LEFT,590),(540,590),(490,BORDER_DOWN_RIGHT)];

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficLightState {
    Red,
    Green,
}

pub struct TrafficLight {
    pub position: Point,
    pub state: TrafficLightState,
    pub timer: u32,
    direction: Direction,
}

impl TrafficLight {
    pub fn reposition_for_entry_lanes() -> Vec<Self> {
        vec![
            TrafficLight::new(Point::new(470, 470), TrafficLightState::Green, Direction::North), // North entry
            TrafficLight::new(Point::new(610, 470), TrafficLightState::Red, Direction::South),   // South entry
            TrafficLight::new(Point::new(470, 610), TrafficLightState::Red, Direction::West),    // West entry
            TrafficLight::new(Point::new(610, 610), TrafficLightState::Green, Direction::East),  // East entry
        ]
    }

    pub fn new(position: Point, initial_state: TrafficLightState, direction: Direction) -> Self {
        Self {
            position,
            state: initial_state,
            timer: 0,
            direction,
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        if self.timer >= 200 { // Reduced cycle time for better flow
            self.timer = 0;
            // Coordinate lights based on direction pairs
            // North-South pair and East-West pair alternate
            match self.direction {
                Direction::North | Direction::South => {
                    self.state = match self.state {
                        TrafficLightState::Red => TrafficLightState::Green,
                        TrafficLightState::Green => TrafficLightState::Red,
                    };
                },
                Direction::East | Direction::West => {
                    self.state = match self.state {
                        TrafficLightState::Green => TrafficLightState::Red,
                        TrafficLightState::Red => TrafficLightState::Green,
                    };
                }
            }
        }
    }

    pub fn is_green(&self) -> bool {
        self.state == TrafficLightState::Green
    }
}

#[allow(dead_code)]
impl Direction {
    /// Generate a random direction
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..=3) {
            0 => Self::North,
            1 => Self::South,
            2 => Self::East,
            _ => Self::West
        }
    }

    /// Generate a random direction but no the choosen one.
    pub fn random_without(other: Self) -> Self {
        let generated =  Self::random();
        if generated == other {
            return Self::random_without(other);
        }
        generated
    }
}

pub fn show_points(canvas: &mut Canvas<Window>) -> Result<(),String> {
    let values = vec![N_S,N_E,N_W,S_N,S_E,S_W,E_W,E_N,E_S,W_E,W_N,W_S];
    for va in values {
        for (i,vb) in va.iter().enumerate() {
            match i {
                0 => canvas.set_draw_color(Color::RED),
                2 => canvas.set_draw_color(Color::CYAN),
                _ => canvas.set_draw_color(Color::GREY),
            }
            let p = Point::new(vb.0, vb.1);
            canvas.draw_rect(Rect::from_center(p, 3, 3))?;
        }
    };
    Ok(())
}

fn get_points(from: Direction,to: Direction) -> Result<(Point,Vec<Point>),String> {
    use Direction::*;
    let points = match (from, to) {
        (North, South) => N_S,
        (North, East) => N_E,
        (North, West) => N_W,

        (South, North) => S_N,
        (South, East) => S_E,
        (South, West) => S_W,

        (East, West) => E_W,
        (East, North) => E_N,
        (East, South) => E_S,

        (West, East) => W_E,
        (West, North) => W_N,
        (West, South) => W_S,

        _ => return Err(String::from("invalid direction combination")),
    };
    Ok((Point::new(points[0].0, points[0].1),vec![Point::new(points[1].0, points[1].1),Point::new(points[2].0, points[2].1)]))
}


pub fn spawn_car<'a>(from: Direction, to: Direction, car_w: u32, car_l: u32) -> Result<Car<'a>, String> {
    let (strt, path) = get_points(from, to)?;
    let color = match (from, to) {
        (Direction::North, Direction::East) | (Direction::South, Direction::West) => Color::YELLOW, // Right turn
        (Direction::North, Direction::West) | (Direction::South, Direction::East) => Color::RED,    // Left turn
        _ => Color::BLUE, // Straight
    };
    let mut car = Car::new(strt, car_w, car_l, color);

    car.set_path(path);
    Ok(car)
}

// Road dimensions
const ROAD_WIDTH: u32 = 200;
const LANE_WIDTH: u32 = ROAD_WIDTH / 2;
const CANVAS_SIZE: u32 = 1080;

// Colors
const ROAD_COLOR: Color = Color::RGB(50, 50, 50);      // Dark gray for road
const LANE_MARKER_COLOR: Color = Color::RGB(255, 255, 255); // White for lane markers

pub fn load_map(canvas: &mut Canvas<Window>) -> Result<(), String> {
    // Set background color (grass)
    canvas.set_draw_color(Color::RGB(34, 139, 34)); // Forest green
    canvas.clear();

    // Draw horizontal road
    canvas.set_draw_color(ROAD_COLOR);
    canvas.fill_rect(Rect::new(0,
        (CANVAS_SIZE as i32 / 2) - (ROAD_WIDTH as i32 / 2),
        CANVAS_SIZE,
        ROAD_WIDTH))?;

    // Draw vertical road
    canvas.fill_rect(Rect::new(
        (CANVAS_SIZE as i32 / 2) - (ROAD_WIDTH as i32 / 2),
        0,
        ROAD_WIDTH,
        CANVAS_SIZE))?;

    // Draw lane markers (horizontal)
    canvas.set_draw_color(LANE_MARKER_COLOR);
    let dash_width: i32 = 30;
    let dash_gap: i32 = 30;
    let center_y = CANVAS_SIZE as i32 / 2;
    let mut x = 0;
    while x < CANVAS_SIZE as i32 {
        canvas.fill_rect(Rect::new(
            x,
            center_y - 2,
            dash_width as u32,
            4,
        ))?;
        x += dash_width + dash_gap;
    }

    // Draw lane markers (vertical)
    let center_x = CANVAS_SIZE as i32 / 2;
    let mut y = 0;
    while y < CANVAS_SIZE as i32 {
        canvas.fill_rect(Rect::new(
            center_x - 2,
            y,
            4,
            dash_width as u32,
        ))?;
        y += dash_width + dash_gap;
    }

    Ok(())
}