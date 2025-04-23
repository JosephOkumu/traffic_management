use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::map::Direction;

#[allow(dead_code)]
pub enum Type {
    Quit,
    SpawnCar(Direction,Direction),
    ToggleDebug,
    None,
}

pub fn handle(ep: &mut EventPump) -> Type {
    for event in ep.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::ESCAPE),
                ..
            } => return Type::Quit,
            Event::KeyDown { keycode: Some(Keycode::D), .. } => return Type::ToggleDebug,
            Event::KeyDown { keycode: Some(Keycode::UP), .. } => return Type::SpawnCar(Direction::North,Direction::random_without(Direction::North)),
            Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => return Type::SpawnCar(Direction::South,Direction::random_without(Direction::South)),
            Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => return Type::SpawnCar(Direction::East,Direction::random_without(Direction::East)),
            Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => return Type::SpawnCar(Direction::West,Direction::random_without(Direction::West)),
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                let spawning_direction = Direction::random();
                return Type::SpawnCar(spawning_direction,Direction::random_without(spawning_direction));
            },
            _ => return Type::None,
        }
    };
    Type::None
}