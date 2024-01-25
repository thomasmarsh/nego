pub mod mcts;
pub mod negamax;

use crate::core::{
    game::{Color, State},
    pieces::PieceTypeId,
    r#move::Move,
};

use minimax::Game;

#[derive(Copy, Clone)]
pub enum AIPlayer {
    Parallel,
    Iterative,
    Mcts,
    Random,
}

impl AIPlayer {
    pub fn step(&self, state: &State, timeout: std::time::Duration) -> Option<State> {
        if Nego::get_winner(state).is_some() {
            return None;
        }

        match self {
            AIPlayer::Parallel => negamax::step_parallel(state, timeout),
            AIPlayer::Iterative => negamax::step_iterative(state, timeout),
            AIPlayer::Mcts => mcts::step(state, timeout),
            AIPlayer::Random => step_random(state),
        }
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
        let b = state.board.black.points();
        let w = state.board.white.points();

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
        state.apply(m);
        None
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
        let p = PieceTypeId::Kunoji4.def();
        (p.lut_offset + p.moves) as u16
    }
}

pub fn step_random(state: &State) -> Option<State> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut ms = Vec::new();
    state.get_moves(&mut ms);
    if ms.is_empty() {
        return None;
    }
    let idx: usize = rng.gen_range(0..ms.len());
    let mut new_state = state.clone();
    new_state.apply(ms[idx]);
    Some(new_state)
}
