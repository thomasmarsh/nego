use crate::core::{
    coord::{ALL_X, ALL_Y, X, Y},
    square::Square,
};

use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not};

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

pub const EMPTY: BitBoard = BitBoard(0);

impl BitBoard {
    /// Construct a new bitboard from a u64
    #[inline]
    pub fn new(b: u64) -> BitBoard {
        BitBoard(b)
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    #[inline]
    pub fn set(x: X, y: Y) -> BitBoard {
        BitBoard::from_square(Square::make_square(x, y))
    }

    #[inline]
    pub fn test(self, x: X, y: Y) -> bool {
        let pos = BitBoard::set(x, y);
        self & pos == pos
    }

    #[inline]
    pub fn test_square(self, square: Square) -> bool {
        BitBoard::from_square(square).is_subset(self)
    }

    #[inline]
    pub fn from_indices(x: usize, y: usize) -> BitBoard {
        BitBoard::set(X::from_index(x), Y::from_index(y))
    }

    #[inline]
    pub fn from_indices_vec(xs: Vec<(usize, usize)>) -> BitBoard {
        xs.iter()
            .fold(EMPTY, |b, (x, y)| b | BitBoard::from_indices(*x, *y))
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    #[inline]
    pub fn from_square(sq: Square) -> BitBoard {
        BitBoard(1u64 << sq.to_int())
    }

    /// Convert an `Option<Square>` to an `Option<BitBoard>`
    #[inline]
    pub fn from_maybe_square(sq: Option<Square>) -> Option<BitBoard> {
        sq.map(BitBoard::from_square)
    }

    /// Convert a `BitBoard` to a `Square`.  This grabs the least-significant `Square`
    #[inline]
    pub fn to_square(self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    /// Count the number of `Squares` set in this `BitBoard`
    #[inline]
    pub fn popcnt(self) -> u32 {
        self.0.count_ones()
    }

    /// Reverse this `BitBoard`.  Look at it from the opponents perspective.
    #[inline]
    pub fn reverse_colors(self) -> BitBoard {
        BitBoard(self.0.swap_bytes())
    }

    /// Convert this `BitBoard` to a `usize` (for table lookups)
    #[inline]
    pub fn to_size(self, rightshift: u8) -> usize {
        (self.0 >> rightshift) as usize
    }

    #[inline]
    pub fn occupied(&self, square: Square) -> bool {
        let mask = BitBoard::from_square(square);
        self & mask == mask
    }

    pub fn is_empty(self) -> bool {
        self == EMPTY
    }

    #[inline]
    pub fn intersects(self, other: BitBoard) -> bool {
        self & other != EMPTY
    }

    #[inline]
    pub fn connected(self, a: BitBoard, b: BitBoard) -> bool {
        self.intersects(a) && self.intersects(b)
    }

    #[inline]
    pub fn is_subset(self, other: BitBoard) -> bool {
        self & other == self
    }

    #[inline]
    pub fn is_disjoint(self, other: BitBoard) -> bool {
        self & other == EMPTY
    }

    pub fn has_opposite_connection(self, start: Square) -> bool {
        // floods areas of 1s until it finds a cross-board connection
        let s = 0xff00000000000000;
        let w = 0x0101010101010101;
        let n = 0x00000000000000ff;
        let e = 0x8080808080808080;

        let path = self.0;
        let mut flood = BitBoard::from_square(start).0 & path;

        // Early exit if sq1 not on fill area
        if flood == 0 {
            return false;
        }

        // With bitboard sq1, do an 4-way flood fill, masking off bits not in
        // path at every step. Stop when fill reaches an opposite connection condition,
        // or fill cannot progress any further
        while flood != 0 {
            let temp: u64 = flood;
            flood |= (flood.wrapping_shl(1) & 0xfefefefefefefefe)
                | (flood.wrapping_shr(1) & 0x7f7f7f7f7f7f7f7f)
                | flood.wrapping_shl(8)
                | flood.wrapping_shr(8);
            flood &= path; // Drop bits not in path
            if (flood & n != 0 && flood & s != 0) || (flood & w != 0 && flood & e != 0) {
                break;
            } else if flood == temp {
                // Fill has stopped
                return false;
            }
        }
        true
    }

    pub fn floodfill4(self, start: Square) -> BitBoard {
        let path = self.0;
        let mut flood = BitBoard::from_square(start).0 & path;

        // Early exit if flood not on fill area
        if flood == 0 {
            return EMPTY;
        }

        // With bitboard flood, do an 4-way flood fill, masking off bits not in
        // path at every step. Stop when fill cannot progress any further
        while flood != 0 {
            let temp: u64 = flood;
            flood |= (flood.wrapping_shl(1) & 0xfefefefefefefefe)
                | (flood.wrapping_shr(1) & 0x7f7f7f7f7f7f7f7f)
                | flood.wrapping_shl(8)
                | flood.wrapping_shr(8);
            flood &= path; // Drop bits not in path
            if flood == temp {
                break;
            } // Fill has stopped
        }
        BitBoard(flood)
    }

    pub fn floodfill8(self, start: Square) -> BitBoard {
        let path = self.0;
        let mut flood = BitBoard::from_square(start).0 & path;

        // Early exit if flood not on fill area
        if flood == 0 {
            return EMPTY;
        }

        // With bitboard flood, do an 8-way flood fill, masking off bits not in
        // path at every step. Stop when fill reaches any set bit in sq2, or
        // fill cannot progress any further
        while flood != 0 {
            let temp: u64 = flood;
            flood |= (flood.wrapping_shl(1) & 0xfefefefefefefefe)
                | (flood.wrapping_shr(1) & 0x7f7f7f7f7f7f7f7f);
            flood |= flood.wrapping_shl(8) | flood.wrapping_shr(8);
            flood &= path; // Drop bits not in path
            if flood == temp {
                break; // Fill has stopped
            }
        }
        BitBoard(flood)
    }

    #[inline]
    pub fn get_adjacent_mask(self) -> BitBoard {
        (self.lshift() | self.rshift() | self.ushift() | self.dshift()) ^ self
    }

    #[inline]
    pub fn from_squares(squares: Vec<Square>) -> Self {
        BitBoard(squares.iter().fold(0, |b, sq| b | (1 << sq.to_index())))
    }

    #[inline]
    pub fn flip_vertical(self) -> BitBoard {
        let x = self.0;
        BitBoard(
            (x << 56)
                | ((x << 40) & 0x00ff000000000000)
                | ((x << 24) & 0x0000ff0000000000)
                | ((x << 8) & 0x000000ff00000000)
                | ((x >> 8) & 0x00000000ff000000)
                | ((x >> 24) & 0x0000000000ff0000)
                | ((x >> 40) & 0x000000000000ff00)
                | (x >> 56),
        )
    }

    #[inline]
    pub fn flip_diag_a1h8(self) -> BitBoard {
        const K1: u64 = 0x5500550055005500;
        const K2: u64 = 0x3333000033330000;
        const K4: u64 = 0x0f0f0f0f00000000;

        let mut x = self.0;
        let mut t = K4 & (x ^ (x << 28));
        x ^= t ^ (t >> 28);
        t = K2 & (x ^ (x << 14));
        x ^= t ^ (t >> 14);
        t = K1 & (x ^ (x << 7));
        x ^= t ^ (t >> 7);
        BitBoard(x)
    }

    // These _by_squares variants for rotating sparse bitboards may not be worth it.
    #[inline]
    pub fn rot90_by_squares(self) -> BitBoard {
        BitBoard::from_squares(self.map(|x| x.rot90_fast32()).collect())
    }

    #[inline]
    pub fn rot180_by_squares(self) -> BitBoard {
        BitBoard::from_squares(self.map(|x| x.rot180()).collect())
    }

    #[inline]
    pub fn rot270_by_squares(self) -> BitBoard {
        BitBoard::from_squares(self.map(|x| x.rot270_fast32()).collect())
    }

    #[inline]
    pub fn rot270(self) -> BitBoard {
        self.flip_diag_a1h8().flip_vertical()
    }

    #[inline]
    pub fn rot90(self) -> BitBoard {
        self.flip_vertical().flip_diag_a1h8()
    }

    #[inline]
    pub fn rot180(self) -> BitBoard {
        const H1: u64 = 0x5555555555555555;
        const H2: u64 = 0x3333333333333333;
        const H4: u64 = 0x0F0F0F0F0F0F0F0F;
        const V1: u64 = 0x00FF00FF00FF00FF;
        const V2: u64 = 0x0000FFFF0000FFFF;
        let mut x = self.0;
        x = ((x >> 1) & H1) | ((x & H1) << 1);
        x = ((x >> 2) & H2) | ((x & H2) << 2);
        x = ((x >> 4) & H4) | ((x & H4) << 4);
        x = ((x >> 8) & V1) | ((x & V1) << 8);
        x = ((x >> 16) & V2) | ((x & V2) << 16);
        x = (x >> 32) | (x << 32);
        BitBoard(x)
    }

    #[inline]
    pub fn rshift(self) -> BitBoard {
        const NOT_H_FILE: BitBoard = BitBoard(0x7f7f7f7f7f7f7f7f); // Excludes the h-file (rightmost column)

        BitBoard((self & NOT_H_FILE).0 << 1)
    }

    #[inline]
    pub fn lshift(self) -> BitBoard {
        const NOT_A_FILE: BitBoard = BitBoard(0xfefefefefefefefe); // Excludes the a-file (leftmost column)
        BitBoard((self & NOT_A_FILE).0 >> 1)
    }

    #[inline]
    pub fn ushift(self) -> BitBoard {
        BitBoard(self.0 >> 8)
    }

    #[inline]
    pub fn dshift(self) -> BitBoard {
        BitBoard(self.0 << 8)
    }

    #[inline]
    pub fn rshiftn(self, n: usize) -> BitBoard {
        let mut b = self;
        for _ in 0..n {
            b = b.rshift();
        }
        b
    }

    #[inline]
    pub fn lshiftn(self, n: usize) -> BitBoard {
        let mut b = self;
        for _ in 0..n {
            b = b.lshift();
        }
        b
    }

    #[inline]
    pub fn ushiftn(self, n: usize) -> BitBoard {
        let mut b = self;
        for _ in 0..n {
            b = b.ushift();
        }
        b
    }

    #[inline]
    pub fn dshiftn(self, n: usize) -> BitBoard {
        let mut b = self;
        for _ in 0..n {
            b = b.dshift();
        }
        b
    }
}

impl fmt::Display for BitBoard {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        s.push_str("   A B C D E F G H\n");
        for y in ALL_Y {
            s.push_str(format!(" {} ", y.to_index() + 1).as_str());
            for x in ALL_X {
                if self.test(x, y) {
                    s.push_str("X ");
                } else {
                    s.push_str(". ");
                }
            }
            s.push_str(format!("{}\n", y.to_index() + 1).as_str());
        }
        s.push_str("   A B C D E F G H\n");
        write!(f, "{}", s)
    }
}

/// For the `BitBoard`, iterate over every `Square` set.
impl Iterator for BitBoard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let result = self.to_square();
            *self ^= BitBoard::from_square(result);
            Some(result)
        }
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

// Impl BitOr
impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

// Impl BitXor

impl BitXor for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

// Impl BitAndAssign

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: &BitBoard) {
        self.0 &= other.0;
    }
}

// Impl BitOrAssign
impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: &BitBoard) {
        self.0 |= other.0;
    }
}

// Impl BitXor Assign
impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: &BitBoard) {
        self.0 ^= other.0;
    }
}

// Impl Mul
impl Mul for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

// Impl Not
impl Not for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}
