use crate::{
    agent::Nego,
    core::game::{Color, State},
};

use minimax::{Game, Strategy};

#[derive(Clone)]
struct Eval;

impl minimax::Evaluator for Eval {
    type G = Nego;

    #[inline]
    fn evaluate(&self, state: &State) -> minimax::Evaluation {
        let b = state.board.black.points() as i16;
        let w = state.board.white.points() as i16;

        const KOMI: i16 = 1;
        let score = b - w - KOMI;
        match state.current {
            Color::Black => score,
            Color::White => -score,
        }
    }
}

pub fn step_parallel(state: &State, timeout: std::time::Duration) -> Option<State> {
    use minimax::{IterativeOptions, ParallelOptions, ParallelSearch};
    let opts = IterativeOptions::new().with_table_byte_size(64_000_000);
    let mut strategy = ParallelSearch::new(Eval, opts.verbose(), ParallelOptions::new());
    strategy.set_max_depth(12);
    strategy.set_timeout(timeout);

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}

pub fn step_negamax(state: &State) -> Option<State> {
    let mut strategy = minimax::Negamax::new(Eval, 4);
    if Nego::get_winner(state).is_some() {
        return None;
    }

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}

pub fn step_iterative(state: &State, timeout: std::time::Duration) -> Option<State> {
    let mut strategy =
        minimax::IterativeSearch::new(Eval, minimax::IterativeOptions::new().verbose());
    strategy.set_timeout(timeout);

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}
