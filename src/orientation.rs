use crate::square::Square;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum Orientation {
    S, // S, 0 (reference orientation of pieces)
    W, // E, 90
    N, // N, 180
    E, // W, 270
}

pub enum Corner {
    NW,
    NE,
    SW,
    SE,
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
