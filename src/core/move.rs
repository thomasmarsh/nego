use crate::core::{
    bitboard::BitBoard,
    coord::{X, Y},
    error::Error,
    move_tab::LUTEntry,
    orientation::Orientation,
    pieces::{PieceId, PieceTypeId},
    square::Square,
};

use serde::Serialize;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Move(u16);

impl Move {
    #[inline]
    pub fn new(piece: PieceId, entry: LUTEntry) -> Move {
        let p = (piece as u16) & 0b1111;
        let e = (entry.0 as u16) << 4;
        Move(e | p)
    }

    #[inline]
    pub fn get_raw_value(self) -> u16 {
        self.0
    }

    #[inline]
    pub fn get_piece(self) -> PieceId {
        PieceId::from_index(self.0 & 0b1111).unwrap()
    }

    pub fn get_lut_entry(self) -> LUTEntry {
        LUTEntry(self.0.wrapping_shr(4) as usize)
    }

    #[inline]
    pub fn position(&self) -> Square {
        self.get_lut_entry().position()
    }

    #[inline]
    pub fn orientation(&self) -> Orientation {
        self.get_lut_entry().orientation()
    }

    #[inline]
    pub fn mask(&self) -> BitBoard {
        self.get_lut_entry().mask()
    }

    // First set bit (self.position may be empty on kunoji types)
    #[inline]
    pub fn to_square(self) -> Square {
        self.mask().to_square()
    }

    #[inline]
    pub fn gaze(&self) -> BitBoard {
        self.get_lut_entry().gaze()
    }

    pub fn notation(&self) -> String {
        format!(
            "{}:{}{:?}",
            self.get_piece().piece_type_id().notation(),
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
        Ok(Move::new(piece, entry))
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
