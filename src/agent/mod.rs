pub mod mcts;
pub mod mcts2;
pub mod negamax;

use crate::core::{
    game::{Color, State},
    r#move::Move,
};

use minimax::Game;

#[derive(Copy, Clone, Debug)]
pub enum Agent {
    Parallel(std::time::Duration),
    Iterative(std::time::Duration),
    Mcts(std::time::Duration),
    Mcts2(std::time::Duration),
    Random,
    Human,
}

impl Agent {
    pub fn is_human(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Agent::Human => true,
            _ => false,
        }
    }

    pub fn step(&self, state: &mut State) -> Option<Move> {
        if Nego::get_winner(state).is_some() {
            return None;
        }

        let result = match self {
            Agent::Parallel(timeout) => negamax::step_parallel(state, *timeout),
            Agent::Iterative(timeout) => negamax::step_iterative(state, *timeout),
            Agent::Mcts(timeout) => mcts::step(state, *timeout),
            Agent::Mcts2(timeout) => mcts2::step(state, *timeout),
            Agent::Random => step_random(state),
            Agent::Human => None,
        };

        if let Some(m) = result {
            state.apply(m);
        }
        result
    }
}

pub struct Nego;

impl minimax::Game for Nego {
    type S = State;
    type M = Move;

    #[inline]
    fn generate_moves(state: &State, moves: &mut Vec<Move>) {
        state.get_moves(moves);
    }

    #[inline]
    fn get_winner(state: &State) -> Option<minimax::Winner> {
        if state.has_moves() {
            return None;
        }

        // We use 0.5 komi to prevent draws
        let komi = 1;
        let b = state.board.black.occupied.popcnt() * 2;
        let w = state.board.white.occupied.popcnt() * 2 + komi;

        if b == w {
            Some(minimax::Winner::Draw)
        } else if b > w && state.current == Color::White {
            Some(minimax::Winner::PlayerJustMoved)
        } else {
            Some(minimax::Winner::PlayerToMove)
        }
    }

    #[inline]
    fn apply(state: &mut State, m: Move) -> Option<State> {
        let mut state = state.clone();
        state.apply(m);
        Some(state)
    }

    #[inline]
    fn zobrist_hash(state: &State) -> u64 {
        state.hash
    }

    fn notation(_: &State, m: Move) -> Option<String> {
        Some(m.notation())
    }

    fn table_index(m: Self::M) -> u16 {
        m.get_raw_value()
    }

    fn max_table_index() -> u16 {
        u16::MAX
        //let p = PieceTypeId::Kunoji4.def();
        //(p.lut_offset + p.moves) as u16
    }
}

pub fn step_random(state: &State) -> Option<Move> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut ms = Vec::new();
    state.get_moves(&mut ms);
    if ms.is_empty() {
        return None;
    }
    let idx: usize = rng.gen_range(0..ms.len());
    Some(ms[idx])
}
