use nego::{agent, core};

#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();

    info!("initializing ray LUT");
    core::ray::Rays::build_lut();

    agent::demo_minimax();
}
