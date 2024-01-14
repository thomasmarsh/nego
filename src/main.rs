mod bitboard;
mod coord;
mod error;
mod game;
mod r#move;
mod move_tab;
mod orientation;
mod pieces;
mod ray;
mod square;
mod zobrist;

use crate::game::State;
use crate::pieces::PieceId;
use crate::r#move::{Color, HasMoves, Move, MoveAccumulator};
use crate::ray::Rays;

use minimax::Strategy;

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
        let mut ma = MoveAccumulator::new();
        state.board.generate_moves(state.current, &mut ma);
        moves.append(&mut ma.0)
    }

    #[inline]
    fn get_winner(state: &State) -> Option<minimax::Winner> {
        let mut hm = HasMoves(false);
        state.board.generate_moves(state.current, &mut hm);
        if hm.0 {
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
        state.place(&m);
        state.current = state.current.next();
        state.update_hash(&m);
        Some(state)
    }

    #[inline]
    fn zobrist_hash(state: &State) -> u64 {
        state.hash
    }

    fn notation(_: &State, m: Move) -> Option<String> {
        let piece = match m.piece {
            PieceId::Boss => "BOS",
            PieceId::Mame => "MAM",
            PieceId::Nobi => "NOB",
            PieceId::Koubaku1 => "KB1",
            PieceId::Koubaku2 => "KB2",
            PieceId::Koubaku3a => "KB3",
            PieceId::Koubaku3b => "KB3",
            PieceId::Kunoji1a => "KJ1",
            PieceId::Kunoji1b => "KJ1",
            PieceId::Kunoji2 => "KJ2",
            PieceId::Kunoji3 => "KJ3",
            PieceId::Kunoji4 => "KJ4",
        };

        let pos = format!(
            "{}{}",
            ((b'A' + m.position().get_x().to_int()) as char),
            m.position().get_y().to_int() as u16 + 1
        );

        Some(format!("{} {}{:?}", piece, pos, m.orientation()))
    }
}

impl State {
    pub fn random_move(&self) -> Option<Move> {
        use rand::Rng;
        let mut ma = MoveAccumulator::new();
        self.board.generate_moves(self.current, &mut ma);
        trace!("moves:");
        for &m in &ma.0 {
            trace!("{:?}", m);
        }
        if ma.0.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..ma.0.len());
            trace!("picked: {:?}", ma.0[i]);
            Some(ma.0[i])
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
        let mut ma = MoveAccumulator::new();
        state.board.generate_moves(state.current, &mut ma);
        if ma.0.is_empty() {
            break;
        }
        println!("Moves:");
        for m in &ma.0 {
            println!("- {:?}", m);
        }

        let idx: usize = rng.gen_range(0..ma.0.len());
        state.place(&ma.0[idx].clone());
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
    strategy.set_timeout(std::time::Duration::from_secs(10));

    // Negamax
    // let mut strategy = minimax::Negamax::new(Eval, 4);

    // MCTS
    // let opts = minimax::MCTSOptions::default().verbose();
    // let mut strategy: minimax::MonteCarloTreeSearch<Nego> =
    // minimax::MonteCarloTreeSearch::new(opts);
    // strategy.set_timeout(Duration::from_secs(10));

    //let mut strategies = [&rand, &iterative];

    let mut state = State::new();
    let mut s = 0;
    loop {
        if Nego::get_winner(&state).is_some() {
            break;
        }
        state.dump();
        match if s == 0 {
            state.random_move()
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
