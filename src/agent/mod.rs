mod mcts;
mod negamax;

use crate::core::{
    game::{Color, State},
    r#move::Move,
};

use minimax::Game;

#[derive(Copy, Clone)]
pub enum AIPlayer {
    Parallel,
    Iterative,
    Mcts,
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
    fn apply(s: &mut State, m: Move) -> Option<State> {
        let mut state = s.clone();
        state.place(m);
        state.current = state.current.next();
        state.update_hash(m);
        Some(state)
    }

    #[inline]
    fn zobrist_hash(state: &State) -> u64 {
        state.hash
    }

    fn notation(_: &State, m: Move) -> Option<String> {
        Some(m.notation())
    }
}

#[allow(unused)]
fn step_random(state: &State) -> Option<State> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut ms = Vec::new();
    state.get_moves(&mut ms);
    if ms.is_empty() {
        return None;
    }
    let idx: usize = rng.gen_range(0..ms.len());
    let mut new_state = state.clone();
    new_state.place(ms[idx]);
    new_state.current = new_state.current.next();

    new_state.board.print_color_map();
    Some(new_state)
}

#[allow(unused)]
pub fn demo_rnd() {
    let mut state = State::new();

    loop {
        if let Some(new_state) = step_random(&state) {
            state = new_state;
        }
    }
}

#[allow(unused)]
pub fn demo_minimax() {
    let mut state = State::new();
    let mut s = 0;
    loop {
        state.dump();
        let new_state_opt = if s == 0 {
            step_random(&state)
        } else {
            negamax::step_iterative(&state, std::time::Duration::from_secs(5))
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
        Nego::get_winner(&state),
        state.board.black.points(),
        state.board.white.points()
    );
    state.dump();
}
