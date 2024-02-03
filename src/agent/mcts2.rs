use std::cmp::Ordering;

use crate::core::{game::Color, game::State, move_tab::LUTEntry, r#move::Move};

use mcts::game::Game;
use mcts::strategies::mcts::TreeSearchStrategy;
use mcts::strategies::Strategy;

type NegoTS = TreeSearchStrategy<Nego>;

pub fn step(state: &State, timeout: std::time::Duration) -> Option<Move> {
    let mut mcts = NegoTS::new();
    mcts.set_verbose();
    mcts.set_timeout(timeout);
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
