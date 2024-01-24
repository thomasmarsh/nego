use crate::core::{
    game::{Color, State},
    r#move::Move,
};

use minimax::{RolloutPolicy, Strategy};
use rand::seq::SliceRandom;

struct Nego;

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

impl State {
    pub fn random_move(&self, rng: &mut impl rand::Rng) -> Option<Move> {
        let mut ms = Vec::new();
        self.get_moves(&mut ms);
        if ms.is_empty() {
            None
        } else {
            let i = rng.gen_range(0..ms.len());
            Some(ms[i])
        }
    }
}

#[derive(Clone)]
struct Eval;

impl minimax::Evaluator for Eval {
    type G = Nego;

    #[inline]
    fn evaluate(&self, state: &State) -> minimax::Evaluation {
        let b = state.board.black.points() as i16;
        let w = state.board.white.points() as i16;

        // We use komi of 1 point here
        let score = b - w - 1;
        match state.current {
            Color::Black => score,
            Color::White => -score,
        }
    }
}

pub fn step_random(state: &State) -> Option<State> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    state.dump();
    let mut ms = Vec::new();
    state.get_moves(&mut ms);
    if ms.is_empty() {
        return None;
    }
    println!("Moves:");
    for m in &ms {
        println!("- {:?}", m);
    }
    let idx: usize = rng.gen_range(0..ms.len());
    let mut new_state = state.clone();
    new_state.place(ms[idx]);
    new_state.current = new_state.current.next();

    new_state.board.print_color_map();
    Some(new_state)
}

#[allow(unused)]
fn demo_rnd() {
    let mut state = State::new();

    loop {
        if let Some(new_state) = step_random(&state) {
            state = new_state;
        }
    }
}

struct Policy;

impl RolloutPolicy for Policy {
    type G = Nego;
    fn random_move(
        &self,
        state: &mut <Nego as minimax::Game>::S,
        moves: &mut Vec<<Nego as minimax::Game>::M>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> <Nego as minimax::Game>::M {
        state.get_moves(moves);
        // TODO:
        // - wieghted random choice
        // - increased weight for advantageous move
        // - prefer moves that block potential connections
        // - prefer moves that connect territory
        // - prefer moves that create territory
        *moves.choose(rng).unwrap()
    }
}

use minimax::Game;

pub fn step_mcts(state: &State) -> Option<State> {
    state.dump();
    use std::time::Duration;
    // MCTS
    let opts = minimax::MCTSOptions::default()
        .verbose()
        .with_rollouts_before_expanding(5);
    let mut strategy: minimax::MonteCarloTreeSearch<Nego> =
        minimax::MonteCarloTreeSearch::new_with_policy(opts, Box::new(Policy));
    strategy.set_timeout(Duration::from_secs(5));

    if Nego::get_winner(state).is_some() {
        return None;
    }

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}

pub fn step_parallel(state: &State) -> Option<State> {
    state.dump();
    use minimax::{IterativeOptions, ParallelOptions, ParallelSearch};
    let opts = IterativeOptions::new().with_table_byte_size(64_000_000);
    //        .with_double_step_increment();
    let mut strategy = ParallelSearch::new(Eval, opts.verbose(), ParallelOptions::new());
    strategy.set_max_depth(12);
    strategy.set_timeout(std::time::Duration::from_secs(90));

    if Nego::get_winner(state).is_some() {
        return None;
    }

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}

pub fn step_negamax(state: &State) -> Option<State> {
    state.dump();
    let mut strategy = minimax::Negamax::new(Eval, 4);
    if Nego::get_winner(state).is_some() {
        return None;
    }

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}

pub fn step_iterative(state: &State) -> Option<State> {
    state.dump();

    let mut strategy =
        minimax::IterativeSearch::new(Eval, minimax::IterativeOptions::new().verbose());
    strategy.set_timeout(std::time::Duration::from_secs(5));

    if Nego::get_winner(state).is_some() {
        return None;
    }

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}

pub fn demo_minimax() {
    let mut state = State::new();
    let mut s = 0;
    loop {
        state.dump();
        let new_state_opt = if s == 0 {
            step_random(&state)
        } else {
            step_iterative(&state)
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
