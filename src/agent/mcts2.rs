use std::cmp::Ordering;
use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::core::{game::Color, game::State, r#move::Move};

use mcts::game::{Game, PlayerIndex};
use mcts::strategies::mcts::select;
use mcts::strategies::mcts::{strategy, SearchConfig, TreeSearch};
use mcts::strategies::Search;

#[derive(Clone, Copy, Default)]
struct NegoStrategy;

type NegoTS = TreeSearch<Nego, strategy::Ucb1TunedDM>;

static MCTS_CELL: OnceLock<Mutex<NegoTS>> = OnceLock::new();

fn build_ts() -> NegoTS {
    NegoTS::default()
        .config(
            SearchConfig::default()
                .expand_threshold(2)
                .max_iterations(usize::MAX)
                .use_transpositions(true)
                .select(select::Ucb1Tuned {
                    exploration_constant: 1.625,
                }),
        )
        .verbose(true)
}

fn get_agent() -> MutexGuard<'static, NegoTS> {
    MCTS_CELL
        .get_or_init(|| Mutex::new(build_ts()))
        .lock()
        .unwrap()
}

pub fn step(state: &State, timeout: std::time::Duration) -> Option<Move> {
    let mut mcts = get_agent();
    mcts.config.max_time = timeout;
    // mcts.strategy.max_iterations = 40000;
    Some(mcts.choose_action(state))
}

impl PlayerIndex for Color {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone)]
struct Nego;

impl Game for Nego {
    type S = State;
    type A = Move;
    type P = Color;

    fn apply(mut state: Self::S, action: &Self::A) -> Self::S {
        state.apply(*action);
        state
    }

    fn generate_actions(state: &Self::S, actions: &mut Vec<Self::A>) {
        state.get_moves(actions);
    }

    fn is_terminal(state: &Self::S) -> bool {
        !state.has_moves()
    }

    fn notation(_: &Self::S, m: &Self::A) -> String {
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

    fn zobrist_hash(state: &Self::S) -> u64 {
        state.hash
    }
}

// TODO for rollouts:
// - wieghted random choice
// - increased weight for advantageous move
// - prefer moves that block potential connections
// - prefer moves that connect territory
// - prefer moves that create territory
// - add randomness for exploration?
