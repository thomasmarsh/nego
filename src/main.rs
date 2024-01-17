mod core;

use crate::core::{
    game::{Color, State},
    r#move::Move,
    ray::Rays,
};

use minimax::{RolloutPolicy, Strategy};
use rand::seq::SliceRandom;

use rustyline::error::ReadlineError;

use log::info;
#[macro_use]
extern crate log;

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
    pub fn random_move(&self) -> Option<Move> {
        use rand::Rng;
        let mut ms = Vec::new();
        self.get_moves(&mut ms);
        trace!("moves:");
        for &m in &ms {
            trace!("{:?}", m);
        }
        if ms.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..ms.len());
            trace!("picked: {:?}", ms[i]);
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
        match state.current {
            Color::Black => b - w,
            Color::White => w - b,
        }
    }
}

#[allow(unused)]
fn demo_rnd() {
    let mut state = State::new();

    use rand::Rng;

    let mut rng = rand::thread_rng();
    loop {
        let mut ms = Vec::new();
        state.get_moves(&mut ms);
        if ms.is_empty() {
            break;
        }
        println!("Moves:");
        for m in &ms {
            println!("- {:?}", m);
        }

        let idx: usize = rng.gen_range(0..ms.len());
        state.place(ms[idx]);
        state.current = state.current.next();

        state.board.print_color_map();
    }
}

fn cli() -> rustyline::Result<()> {
    use rustyline::*;
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("nego> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                println!("Line: {:?}", line.trim());
                if line.trim() == "exit" {
                    break;
                }
                if line.trim() == "demo" {
                    demo_minimax();
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
    Ok(())
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

fn demo_minimax() {
    use minimax::Game;
    use minimax::{IterativeOptions, IterativeSearch, ParallelOptions, ParallelSearch};

    let opts = IterativeOptions::new().with_table_byte_size(64_000_000);
    // .with_double_step_increment();

    use std::time::Duration;
    // let mut strategy = ParallelSearch::new(Eval, opts.verbose(), ParallelOptions::new());
    // strategy.set_max_depth(12);
    // strategy.set_timeout(std::time::Duration::from_secs(90));

    // Iterative
    let mut strategy =
        minimax::IterativeSearch::new(Eval, minimax::IterativeOptions::new().verbose());
    strategy.set_timeout(std::time::Duration::from_secs(60));

    // Negamax
    // let mut strategy = minimax::Negamax::new(Eval, 4);

    // MCTS
    let opts = minimax::MCTSOptions::default()
        .verbose()
        .with_rollouts_before_expanding(5);
    let mut mcts: minimax::MonteCarloTreeSearch<Nego> =
        minimax::MonteCarloTreeSearch::new_with_policy(opts, Box::new(Policy));
    mcts.set_timeout(Duration::from_secs(60));

    //let mut strategies = [&rand, &iterative];

    let mut state = State::new();
    let mut s = 0;
    loop {
        if Nego::get_winner(&state).is_some() {
            break;
        }
        state.dump();
        match if s == 0 {
            mcts.choose_move(&state)
            // state.random_move()
        } else {
            strategy.choose_move(&state)
            // state.random_move()
        } {
            Some(m) => {
                trace!("MAIN: APPLY: {:?}", m);
                state = Nego::apply(&mut state, m).unwrap();
            }
            None => break,
        }
        s = 1 - s;
    }
    println!(
        "Winner: {:?} (b={}, w={})",
        Nego::get_winner(&state),
        state.board.black.points(),
        state.board.white.points()
    );
    state.dump();
}

fn main() -> rustyline::Result<()> {
    pretty_env_logger::init();

    info!("initializing ray LUT");
    Rays::build_lut();

    // cli()

    //dem();
    demo_minimax();
    Ok(())
}
