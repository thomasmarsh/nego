use crate::core::{error::Error, square::Square};

use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum Orientation {
    S, // S, 0 (reference orientation of pieces)
    W, // E, 90
    N, // N, 180
    E, // W, 270
}

pub const NUM_ORIENTATIONS: usize = 4;

pub const ALL_ORIENTATIONS: [Orientation; 4] = [
    Orientation::S,
    Orientation::W,
    Orientation::N,
    Orientation::E,
];

impl Orientation {
    #[inline]
    pub fn at_limit(&self, square: Square) -> bool {
        use Orientation::*;
        match self {
            S => square.is_bottom_edge(),
            W => square.is_left_edge(),
            N => square.is_top_edge(),
            E => square.is_right_edge(),
        }
    }

    #[inline]
    pub fn opposite(&self) -> Orientation {
        use Orientation::*;
        match self {
            S => N,
            W => E,
            N => S,
            E => W,
        }
    }

    // if "facing" one direction, what direction is to the right?
    #[inline]
    pub fn right(&self) -> Orientation {
        use Orientation::*;
        match self {
            S => W,
            W => N,
            N => E,
            E => S,
        }
    }

    // if "facing" one direction, what direction is to the left?
    #[inline]
    pub fn left(&self) -> Orientation {
        use Orientation::*;
        match self {
            S => E,
            W => S,
            N => W,
            E => N,
        }
    }

    #[inline]
    pub fn from_index(i: u8) -> Orientation {
        match i {
            0 => Orientation::S,
            1 => Orientation::W,
            2 => Orientation::N,
            3 => Orientation::E,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Orientation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::InvalidRank);
        }
        match s.chars().next().unwrap() {
            'S' => Ok(Orientation::S),
            'W' => Ok(Orientation::W),
            'N' => Ok(Orientation::N),
            'E' => Ok(Orientation::E),
            _ => Err(Error::InvalidOrientation),
        }
    }
}
