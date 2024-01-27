use crate::{
    agent::Nego,
    core::{
        game::{Color, State},
        r#move::Move,
    },
};

use minimax::{IterativeOptions, IterativeSearch, ParallelOptions, ParallelSearch, Strategy};

use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Clone)]
struct Eval;

impl minimax::Evaluator for Eval {
    type G = Nego;

    #[inline]
    fn evaluate(&self, state: &State) -> minimax::Evaluation {
        let b = state.board.black.occupied.popcnt() as i16;
        let w = state.board.black.occupied.popcnt() as i16;

        const KOMI: i16 = 1;
        let score = (b - w) * 2 - KOMI;
        match state.current {
            Color::Black => score,
            Color::White => -score,
        }
    }
}

fn iterative_opts() -> IterativeOptions {
    IterativeOptions::new()
        .with_table_byte_size(64_000)
        .with_mtdf()
        .with_singular_extension()
        // TODO: adding countermoves triggers a panic.
        //
        // Message:  index out of bounds: the len is 1725 but the index is 1889
        // Location: /Users/tmarsh/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minimax-0.5.3/src/strategies/table.rs:407
        //
        //.with_countermoves()
        //.with_countermove_history()
        .verbose()
}

fn parallel_opts() -> ParallelOptions {
    ParallelOptions::new()
}

static PARALLEL_CELL: OnceLock<Mutex<ParallelSearch<Eval>>> = OnceLock::new();
static ITERATIVE_CELL: OnceLock<Mutex<IterativeSearch<Eval>>> = OnceLock::new();

fn get_parallel_agent() -> MutexGuard<'static, ParallelSearch<Eval>> {
    PARALLEL_CELL
        .get_or_init(|| Mutex::new(ParallelSearch::new(Eval, iterative_opts(), parallel_opts())))
        .lock()
        .unwrap()
}

fn get_iterative_agent() -> MutexGuard<'static, IterativeSearch<Eval>> {
    ITERATIVE_CELL
        .get_or_init(|| Mutex::new(IterativeSearch::new(Eval, iterative_opts())))
        .lock()
        .unwrap()
}

pub fn step<S>(
    state: &State,
    timeout: std::time::Duration,
    strategy: &mut MutexGuard<'static, S>,
) -> Option<Move>
where
    S: Strategy<Nego>,
{
    strategy.set_timeout(timeout);
    strategy.choose_move(state)
}

pub fn step_iterative(state: &State, timeout: std::time::Duration) -> Option<Move> {
    step(state, timeout, &mut get_iterative_agent())
}

pub fn step_parallel(state: &State, timeout: std::time::Duration) -> Option<Move> {
    step(state, timeout, &mut get_parallel_agent())
}
