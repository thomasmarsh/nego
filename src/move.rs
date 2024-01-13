use crate::bitboard::BitBoard;
use crate::move_tab::LUTEntry;
use crate::orientation::Orientation;
use crate::pieces::PieceId;
use crate::square::Square;

use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn next(&self) -> Color {
        use Color::*;
        match self {
            Black => White,
            White => Black,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub color: Color,    // 1 bit
    pub piece: PieceId,  // 4 bit
    pub entry: LUTEntry, // 11 bit
}

impl Move {
    pub fn position(&self) -> Square {
        self.entry.position()
    }

    pub fn orientation(&self) -> Orientation {
        self.entry.orientation()
    }

    pub fn mask(&self) -> BitBoard {
        self.entry.mask()
    }

    pub fn gaze(&self) -> BitBoard {
        self.entry.gaze()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}{:?}",
            self.piece.piece_type_id().notation(),
            self.position().to_string().to_uppercase(),
            self.orientation()
        )
    }
}

pub trait MoveVisitor {
    fn visit(&mut self, m: Move);
    fn bailout(&self) -> bool {
        false
    }
}

pub struct HasMoves(pub bool);

impl MoveVisitor for HasMoves {
    fn visit(&mut self, _: Move) {
        self.0 = true;
    }

    fn bailout(&self) -> bool {
        self.0
    }
}

pub struct MoveCounter(pub usize);

impl MoveVisitor for MoveCounter {
    fn visit(&mut self, _: Move) {
        self.0 += 1;
    }
}

pub struct MoveAccumulator(pub Vec<Move>);

impl MoveAccumulator {
    pub fn new() -> Self {
        Self(Vec::with_capacity(1720))
    }
}

impl MoveVisitor for MoveAccumulator {
    fn visit(&mut self, m: Move) {
        self.0.push(m);
    }
}
