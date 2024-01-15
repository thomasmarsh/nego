use crate::bitboard::BitBoard;
use crate::coord::{X, Y};
use crate::error::Error;
use crate::move_tab::LUTEntry;
use crate::orientation::Orientation;
use crate::pieces::{PieceId, PieceTypeId};
use crate::square::Square;

use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    #[inline]
    pub fn next(&self) -> Color {
        use Color::*;
        match self {
            Black => White,
            White => Black,
        }
    }
}

// TODO: struct PackedMove(u16)

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub piece: PieceId,  // 4 bit
    pub entry: LUTEntry, // 11 bit
}

impl Move {
    #[inline]
    pub fn position(&self) -> Square {
        self.entry.position()
    }

    #[inline]
    pub fn orientation(&self) -> Orientation {
        self.entry.orientation()
    }

    #[inline]
    pub fn mask(&self) -> BitBoard {
        self.entry.mask()
    }

    // First set bit (self.position may be empty on kunoji types)
    #[inline]
    pub fn to_square(self) -> Square {
        self.mask().to_square()
    }

    #[inline]
    pub fn gaze(&self) -> BitBoard {
        self.entry.gaze()
    }

    pub fn notation(&self) -> String {
        format!(
            "{}:{}{:?}",
            self.piece.piece_type_id().notation(),
            self.position().to_string().to_uppercase(),
            self.orientation()
        )
        .to_string()
    }

    pub fn parse(s: &str) -> Result<Move, Error> {
        if s.len() != 8 {
            return Err(Error::InvalidFormat);
        }
        if s.as_bytes()[4] != b':' {
            return Err(Error::InvalidFormat);
        }

        fn parse_piece_type(piece_str: &str) -> Result<PieceTypeId, Error> {
            match piece_str {
                "BOS" => Ok(PieceTypeId::Boss),
                "MAM" => Ok(PieceTypeId::Mame),
                "NOB" => Ok(PieceTypeId::Nobi),
                "KB1" => Ok(PieceTypeId::Koubaku1),
                "KB2" => Ok(PieceTypeId::Koubaku2),
                "KB3" => Ok(PieceTypeId::Koubaku3),
                "KJ1" => Ok(PieceTypeId::Kunoji1),
                "KJ2" => Ok(PieceTypeId::Kunoji2),
                "KJ3" => Ok(PieceTypeId::Kunoji3),
                "KJ4" => Ok(PieceTypeId::Kunoji4),
                _ => Err(Error::InvalidPiece),
            }
        }

        let piece_type = parse_piece_type(&s[0..4])?;
        let x = X::from_str(&s.as_bytes()[5].to_string())?;
        let y = Y::from_str(&s.as_bytes()[6].to_string())?;
        let position = Square::make_square(x, y);
        let orientation = Orientation::from_str(&s.as_bytes()[7].to_string())?;

        let piece = match piece_type {
            PieceTypeId::Boss => PieceId::Boss,
            PieceTypeId::Mame => PieceId::Mame,
            PieceTypeId::Nobi => PieceId::Nobi,
            PieceTypeId::Koubaku1 => PieceId::Koubaku1,
            PieceTypeId::Koubaku2 => PieceId::Koubaku2,
            PieceTypeId::Koubaku3 => PieceId::Koubaku3a, // may need reindexing
            PieceTypeId::Kunoji1 => PieceId::Kunoji1a,   // may need reindexing
            PieceTypeId::Kunoji2 => PieceId::Kunoji2,
            PieceTypeId::Kunoji3 => PieceId::Kunoji3,
            PieceTypeId::Kunoji4 => PieceId::Kunoji4,
        };

        let entry =
            LUTEntry::lookup(piece_type, position, orientation).ok_or(Error::LUTEntryNotFound)?;
        Ok(Move { piece, entry })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.notation())
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Move::parse(s)
    }
}

pub trait MoveVisitor {
    fn visit(&mut self, m: Move);

    #[inline]
    fn bailout(&self) -> bool {
        false
    }
}

pub struct HasMoves(pub bool);

impl MoveVisitor for HasMoves {
    #[inline]
    fn visit(&mut self, _: Move) {
        self.0 = true;
    }

    #[inline]
    fn bailout(&self) -> bool {
        self.0
    }
}

pub struct MoveCounter(pub usize);

impl MoveVisitor for MoveCounter {
    #[inline]
    fn visit(&mut self, _: Move) {
        self.0 += 1;
    }
}

pub struct MoveAccumulator(pub Vec<Move>);

impl MoveAccumulator {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::with_capacity(1720 >> 1))
    }
}

impl MoveVisitor for MoveAccumulator {
    #[inline]
    fn visit(&mut self, m: Move) {
        self.0.push(m);
    }
}
