# Traffic Intersection Simulator

A real-time traffic intersection simulation implemented in Rust using SDL2. The project simulates traffic flow at a four-way intersection with intelligent traffic light management and vehicle routing.

## Project Overview

This simulation models a traffic intersection with the following key features:

- Two intersecting roads with one lane in each direction
- Traffic lights controlling vehicle flow
- Vehicles with different routing behaviors (left turn, right turn, straight)
- Intelligent traffic management system to prevent congestion
- Real-time user interaction for spawning vehicles

## Prerequisites

- Rust (latest stable version)
- SDL2 development libraries

### Installing SDL2

#### Ubuntu/Debian
```bash
sudo apt-get install libsdl2-dev
```

#### macOS
```bash
brew install sdl2
```

#### Windows
Download SDL2 development libraries from [SDL2's website](https://www.libsdl.org/download-2.0.php) and set up according to their documentation.

## Building and Running

1. Clone the repository:
```bash
git clone https://github.com/yourusername/traffic-intersection
cd traffic-intersection
```

2. Build and run:
```bash
cargo run --release
```

## Controls

- **↑ Up Arrow**: Spawn vehicle from South
- **↓ Down Arrow**: Spawn vehicle from North
- **→ Right Arrow**: Spawn vehicle from West
- **← Left Arrow**: Spawn vehicle from East
- **R**: Spawn vehicle from random direction
- **ESC**: Exit simulation


### Traffic Rules

1. **Fixed Velocity**: All vehicles maintain a constant speed
2. **Safety Distance**: Vehicles maintain a safe following distance
3. **Traffic Light Compliance**: Vehicles stop at red lights and proceed on green
4. **No Route Changes**: Vehicles must follow their predetermined route
5. **Congestion Control**: System prevents traffic buildup (max 7 vehicles per lane)

## Traffic Light System

The intersection uses a two-color (Red/Green) traffic light system positioned at each lane entry point. The traffic management algorithm prioritizes:

1. Preventing collisions
2. Minimizing congestion
3. Optimizing flow through the intersection

## Project Structure

```
.
├── src/
│   ├── main.rs         # Application entry point
│   ├── map.rs          # Intersection layout and rendering
│   ├── events.rs       # Event handling
│   ├── cars/
│   │   ├── mod.rs      # Vehicle module definitions
│   │   └── car.rs      # Vehicle behavior implementation
│   └── entities/
│       ├── mod.rs      # Game entity definitions
│       └── objects.rs   # Core game objects
├── assets/             # Game assets (sprites, etc.)
├── Cargo.toml          # Project dependencies
└── README.md          # This file
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.


### Contributors
- [Joseph Otieno](https://learn.zone01kisumu.ke/git/josotieno)
- [Malkika Asman](https://learn.zone01kisumu.ke/git/masman)
- [Kaunda Rodgers](https://learn.zone01kisumu.ke/git/krodgers)