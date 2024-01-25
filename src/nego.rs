mod agent;
mod core;

#[macro_use]
extern crate log;

fn main() -> rustyline::Result<()> {
    pretty_env_logger::init();

    info!("initializing ray LUT");
    core::ray::Rays::build_lut();

    agent::demo_minimax();
    Ok(())
}
