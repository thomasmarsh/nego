use std::cmp::Ordering;
use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::core::{game::Color, game::State, r#move::Move};

use mcts::game::Game;
use mcts::strategies::mcts::TreeSearch;
use mcts::strategies::Strategy;

type NegoTS = TreeSearch<Nego>;

static MCTS_CELL: OnceLock<Mutex<NegoTS>> = OnceLock::new();

fn get_agent() -> MutexGuard<'static, NegoTS> {
    MCTS_CELL
        .get_or_init(|| {
            let mut mcts = NegoTS::new();
            mcts.set_verbose();
            Mutex::new(mcts)
        })
        .lock()
        .unwrap()
}

pub fn step(state: &State, timeout: std::time::Duration) -> Option<Move> {
    let mut mcts = get_agent();
    // mcts.set_timeout(timeout);
    mcts.set_max_rollouts(40000);
    mcts.choose_move(state)
}

struct Nego;

impl Game for Nego {
    type S = State;
    type M = Move;
    type P = Color;

    fn apply(init_state: &Self::S, m: Self::M) -> Self::S {
        let mut state = init_state.clone();
        state.apply(m);
        state
    }

    fn gen_moves(state: &Self::S) -> Vec<Self::M> {
        let mut moves = Vec::new();
        state.get_moves(&mut moves);
        moves
    }

    fn is_terminal(state: &Self::S) -> bool {
        !state.has_moves()
    }

    fn notation(_: &Self::S, m: &Self::M) -> String {
        m.notation()
    }

    fn winner(state: &Self::S) -> Option<Self::P> {
        if state.has_moves() {
            return None;
        }

        // We use 0.5 komi to prevent draws
        let komi = 1;
        let b = state.board.black.occupied.popcnt() * 2;
        let w = state.board.white.occupied.popcnt() * 2 + komi;

        match b.cmp(&w) {
            Ordering::Greater => Some(Color::Black),
            Ordering::Less => Some(Color::White),
            Ordering::Equal => None,
        }
    }

    fn player_to_move(state: &Self::S) -> Self::P {
        state.current
    }
}

// TODO for rollouts:
// - wieghted random choice
// - increased weight for advantageous move
// - prefer moves that block potential connections
// - prefer moves that connect territory
// - prefer moves that create territory
// - add randomness for exploration?
