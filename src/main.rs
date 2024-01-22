use std::time::{Duration, Instant};

fn main() {
    let config = quarkstrom::Config {
        view_size: 512.0,
        window_mode: quarkstrom::WindowMode::Windowed(1280, 720),
    };
    quarkstrom::run::<Renderer>(config);

    // Assign a value to cap the tps
    let tps_cap: Option<u32> = None;

    let desired_frame_time = tps_cap
        .map(|tps| Duration::from_secs_f64(1.0 / tps as f64));

    let mut simulation = Simulation::new();

    loop {
        let frame_timer = Instant::now();

        simulation.update();
        simulation.convert();

        // Cap tps
        if let Some(desired_frame_time) = desired_frame_time {
            while frame_timer.elapsed() < desired_frame_time {}
        }
    }
}

#[derive(Clone)]
enum Boundary {

}

#[derive(Clone)]
struct Renderer {

}

impl Renderer {

}

struct World {

}

#[derive(Clone)]
struct Particle {

}

struct Simulation {

}

impl Simulation {

}

struct Grid {

}

impl Grid {

}