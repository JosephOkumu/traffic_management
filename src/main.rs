mod entities;
mod events;
mod cars;
mod map;

use std::time::Duration;
use cars::UpdateState;
use sdl2::{image::LoadTexture, pixels::Color, video::Window};

use entities::Entity;

const SCREEN_SIZE: (u32,u32) = (1100,1100);
const COOLDOWN_MS: u64 = 450;
const TICK_SPEED: u32 = 60;
const VELOCITY_P_S:u32 = 240;
// const DEBUG: bool = true;

fn init_window(sdl_context: sdl2::Sdl) -> Result<Window,String> {
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Affichage d'une partie de l'image", SCREEN_SIZE.0, SCREEN_SIZE.1)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    Ok(window)
}


fn main() -> Result<(), String> {
    // let mut ctrl = entities::Controller::init();

    let sdl_context = sdl2::init()?;
    let window = init_window(sdl_context.clone())?;
    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut cars: Vec<cars::Car> = Vec::new();

    let mut cooldown_now = std::time::Instant::now();
    let mut tick_time = std::time::Instant::now();

    let mut car_spawned = 0u32;
    let mut car_passed: u32 = 0u32;

    let mut collisions_count: u32 = 0u32;

    let mut debug = false;

    let tc = canvas.texture_creator();
    let car_texture = tc.load_texture("assets/cars.png")?;


    'running: loop {
        match events::handle(&mut event_pump)
        {
            events::Type::SpawnCar(from,to) => {
                if cooldown_now.elapsed() >= Duration::from_millis(COOLDOWN_MS) {
                    let mut tmp = map::spawn_car(from, to, 32,45).unwrap();
                    tmp.set_texture(&car_texture);
                    tmp.set_velocity(VELOCITY_P_S/TICK_SPEED);
                    cars.push(tmp);
                    cooldown_now = std::time::Instant::now();
                    car_spawned += 1;
                }
            },
            events::Type::Quit => break 'running,
            events::Type::ToggleDebug => {
                debug= !debug;
            }
            _ => {},
        };

        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();

        map::load_map(&mut canvas)?;
        // map::show_points(&mut canvas)?;


        let cars_clone = cars.clone();
        let mut to_remove = -1;
        for (i,c) in cars.iter_mut().enumerate() {
            let mut tmp = cars_clone.clone();
            tmp.remove(i);
            match  c.update(tmp) {
                UpdateState::Finished => to_remove = i as i32,
                UpdateState::Waiting => {
                    match (debug,c.get_detections()) {
                        (true,Some((l,u))) => {
                            canvas.set_draw_color(Color::RED);
                            canvas.draw_rect(l).unwrap();
                            canvas.draw_rect(u).unwrap();
                        },
                        _ => {},
                    };
                },
                UpdateState::Collided => {
                    collisions_count += 1;
                    to_remove = i as i32;
                },
                _ => {
                    match (debug,c.get_detections()) {
                        (true,Some((l,u))) => {
                            canvas.set_draw_color(Color::GRAY);
                            canvas.draw_rect(l).unwrap();
                            canvas.draw_rect(u).unwrap();
                        },
                        _ => {},
                    };
                },
            }
            c.set_debug(debug);
            c.display(&mut canvas).unwrap();
        }
        if to_remove != -1 {
            car_passed += 1;
            cars.remove(to_remove as usize);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, tick_time.elapsed().as_nanos() as u32 / TICK_SPEED));
        tick_time = std::time::Instant::now();
    }

    println!("\ncar spawned : {}\ncar passed  : {}\ncollisions  : {}\n",car_passed,car_spawned,collisions_count);
    Ok(())
}
