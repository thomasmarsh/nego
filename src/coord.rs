use crate::error::Error;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
#[repr(u8)]
pub enum X {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
}

pub const ALL_X: [X; 8] = [X::X0, X::X1, X::X2, X::X3, X::X4, X::X5, X::X6, X::X7];
pub const ALL_Y: [Y; 8] = [Y::Y0, Y::Y1, Y::Y2, Y::Y3, Y::Y4, Y::Y5, Y::Y6, Y::Y7];

impl X {
    /// Convert a `usize` into a `Rank` (the inverse of to_index).  If the number is > 7, wrap
    /// around.
    #[inline]
    pub fn from_index(i: usize) -> X {
        // match is optimized to no-op with opt-level=1 with rustc 1.53.0
        match i & 7 {
            0 => X::X0,
            1 => X::X1,
            2 => X::X2,
            3 => X::X3,
            4 => X::X4,
            5 => X::X5,
            6 => X::X6,
            7 => X::X7,
            _ => unreachable!(),
        }
    }

    /// Go one file to the left.  If impossible, wrap around.
    #[inline]
    pub fn left(&self) -> X {
        self.leftn(1)
    }

    /// Go one file to the right.  If impossible, wrap around.
    #[inline]
    pub fn right(&self) -> X {
        self.rightn(1)
    }

    /// Go one file to the left.  If impossible, wrap around.
    #[inline]
    pub fn leftn(&self, n: u8) -> X {
        X::from_index(self.to_index().wrapping_sub(n as usize))
    }

    /// Go one file to the right.  If impossible, wrap around.
    #[inline]
    pub fn rightn(&self, n: u8) -> X {
        X::from_index(self.to_index() + n as usize)
    }

    /// Convert this `X` into a `usize` between 0 and 7 (inclusive).
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_int(&self) -> u8 {
        *self as u8
    }
}

impl FromStr for X {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidRank);
        }
        match s.chars().next().unwrap() {
            '1' => Ok(X::X0),
            '2' => Ok(X::X1),
            '3' => Ok(X::X2),
            '4' => Ok(X::X3),
            '5' => Ok(X::X4),
            '6' => Ok(X::X5),
            '7' => Ok(X::X6),
            '8' => Ok(X::X7),
            _ => Err(Error::InvalidRank),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
#[repr(u8)]
pub enum Y {
    Y0,
    Y1,
    Y2,
    Y3,
    Y4,
    Y5,
    Y6,
    Y7,
}

impl Y {
    /// Convert a `usize` into a `Rank` (the inverse of to_index).  If the number is > 7, wrap
    /// around.
    #[inline]
    pub fn from_index(i: usize) -> Y {
        // match is optimized to no-op with opt-level=1 with rustc 1.53.0
        match i & 7 {
            0 => Y::Y0,
            1 => Y::Y1,
            2 => Y::Y2,
            3 => Y::Y3,
            4 => Y::Y4,
            5 => Y::Y5,
            6 => Y::Y6,
            7 => Y::Y7,
            _ => unreachable!(),
        }
    }

    /// Go one rank down.  If impossible, wrap around.
    #[inline]
    pub fn up(&self) -> Y {
        self.upn(1)
    }

    #[inline]
    pub fn upn(&self, n: u8) -> Y {
        Y::from_index(self.to_index().wrapping_sub(n as usize))
    }

    /// Go one file up.  If impossible, wrap around.
    #[inline]
    pub fn down(&self) -> Y {
        self.downn(1)
    }

    #[inline]
    pub fn downn(&self, n: u8) -> Y {
        Y::from_index(self.to_index() + n as usize)
    }

    /// Convert this `Y` into a `usize` from 0 to 7 inclusive.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_int(&self) -> u8 {
        *self as u8
    }
}

impl FromStr for Y {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidRank);
        }
        match s.chars().next().unwrap() {
            '1' => Ok(Y::Y0),
            '2' => Ok(Y::Y1),
            '3' => Ok(Y::Y2),
            '4' => Ok(Y::Y3),
            '5' => Ok(Y::Y4),
            '6' => Ok(Y::Y5),
            '7' => Ok(Y::Y6),
            '8' => Ok(Y::Y7),
            _ => Err(Error::InvalidRank),
        }
    }
}
