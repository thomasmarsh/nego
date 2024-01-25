use nego::{agent, core::game, core::ray::Rays};

use minimax::Game;

#[macro_use]
extern crate log;

pub fn demo_minimax() {
    let mut state = game::State::new();
    let mut s = 0;

    let timeout = std::time::Duration::from_secs(60);
    loop {
        state.dump();
        let new_state_opt = if s == 0 {
            agent::AIPlayer::Random.step(&state, timeout)
        } else {
            agent::AIPlayer::Iterative.step(&state, timeout)
        };
        s = 1 - s;

        if let Some(new_state) = new_state_opt {
            state = new_state;
        } else {
            break;
        }
    }
    println!(
        "Winner: {:?} (b={}, w={})",
        agent::Nego::get_winner(&state),
        state.board.black.points(),
        state.board.white.points()
    );
    state.dump();
}

fn main() {
    pretty_env_logger::init();

    info!("initializing ray LUT");
    Rays::build_lut();

    demo_minimax();
}
