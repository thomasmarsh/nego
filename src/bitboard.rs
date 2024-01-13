use crate::coord::{ALL_X, ALL_Y, X, Y};
use crate::square::Square;

use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not};

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

#[allow(dead_code)]
pub const EMPTY: BitBoard = BitBoard(0);

const NOT_A_FILE: BitBoard = BitBoard(0xfefefefefefefefe); // Excludes the a-file (leftmost column)
const NOT_H_FILE: BitBoard = BitBoard(0x7f7f7f7f7f7f7f7f); // Excludes the h-file (rightmost column)

const NEIGHBOR_MASK: [u64; 64] = [
    0x0000000000000102,
    0x0000000000000205,
    0x000000000000040a,
    0x0000000000000814,
    0x0000000000001028,
    0x0000000000002050,
    0x00000000000040a0,
    0x0000000000008040,
    0x0000000000010201,
    0x0000000000020502,
    0x0000000000040a04,
    0x0000000000081408,
    0x0000000000102810,
    0x0000000000205020,
    0x000000000040a040,
    0x0000000000804080,
    0x0000000001020100,
    0x0000000002050200,
    0x00000000040a0400,
    0x0000000008140800,
    0x0000000010281000,
    0x0000000020502000,
    0x0000000040a04000,
    0x0000000080408000,
    0x0000000102010000,
    0x0000000205020000,
    0x000000040a040000,
    0x0000000814080000,
    0x0000001028100000,
    0x0000002050200000,
    0x00000040a0400000,
    0x0000008040800000,
    0x0000010201000000,
    0x0000020502000000,
    0x0000040a04000000,
    0x0000081408000000,
    0x0000102810000000,
    0x0000205020000000,
    0x000040a040000000,
    0x0000804080000000,
    0x0001020100000000,
    0x0002050200000000,
    0x00040a0400000000,
    0x0008140800000000,
    0x0010281000000000,
    0x0020502000000000,
    0x0040a04000000000,
    0x0080408000000000,
    0x0102010000000000,
    0x0205020000000000,
    0x040a040000000000,
    0x0814080000000000,
    0x1028100000000000,
    0x2050200000000000,
    0x40a0400000000000,
    0x8040800000000000,
    0x0201000000000000,
    0x0502000000000000,
    0x0a04000000000000,
    0x1408000000000000,
    0x2810000000000000,
    0x5020000000000000,
    0xa040000000000000,
    0x4080000000000000,
];

impl BitBoard {
    #[inline]
    pub fn empty() -> Self {
        Self(0)
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
        if !self.test_square(start) {
            return false;
        }
        let s = 0xff00000000000000;
        let w = 0x0101010101010101;
        let n = 0x00000000000000ff;
        let e = 0x8080808080808080;

        let mut flood = 1u64 << start.to_index() as u64;
        let mut queue = flood;

        while queue != 0 {
            let mut next_queue = 0;
            for (i, mask) in NEIGHBOR_MASK.iter().enumerate() {
                if (queue & (1u64 << i)) != 0 {
                    let neighbors = mask & self.0 & !flood;
                    flood |= neighbors;
                    next_queue |= neighbors;
                    if (flood & n != 0 && flood & s != 0) || (flood & w != 0 && flood & e != 0) {
                        return true;
                    }
                }
            }
            queue = next_queue;
        }
        false
    }

    // This and the previous fn need testing
    pub fn floodfill(self, start: Square) -> BitBoard {
        // floods areas of 0s
        if self.test_square(start) {
            return BitBoard(0);
        }

        let mut flood = 1u64 << start.to_index() as u64;
        let mut queue = flood;

        while queue != 0 {
            let mut next_queue = 0;
            for (i, mask) in NEIGHBOR_MASK.iter().enumerate() {
                if (queue & (1u64 << i)) != 0 {
                    let neighbors = mask & !self.0 & !flood;
                    flood |= neighbors;
                    next_queue |= neighbors;
                }
            }
            queue = next_queue;
        }
        BitBoard(flood)
    }

    #[inline]
    pub fn touches_wall(self) -> bool {
        let walls_mask: u64 = 0xff818181818181ff;
        self.0 & walls_mask != 0
    }

    #[inline]
    pub fn is_adjacent(self, other: BitBoard) -> bool {
        let s = self.dshift();
        let w = self.lshift();
        let n = self.ushift();
        let e = self.rshift();
        (s | w | n | e) & other != EMPTY
    }

    #[inline]
    pub fn find_group(self, pos: Square) -> BitBoard {
        (!self).floodfill(pos)
    }

    #[inline]
    pub fn get_adjacent_mask(self) -> BitBoard {
        (self.lshift() | self.rshift() | self.ushift() | self.dshift()) ^ self
    }

    pub fn from_squares(squares: Vec<Square>) -> Self {
        let mut b = BitBoard(0);
        for sq in squares {
            b.0 |= 1 << sq.to_index();
        }
        b
    }

    #[inline]
    pub fn to_squares(self) -> Vec<Square> {
        let mut indices = Vec::with_capacity(4);
        let mut num = self.0;
        while num != 0 {
            let tz = num.trailing_zeros();
            indices.push(Square::new(tz as u8));
            num &= !(1 << tz);
        }
        indices
    }

    #[inline]
    pub fn to_squares_rev(self) -> Vec<Square> {
        let mut indices = Vec::with_capacity(4);
        let mut num = self.0;
        while num != 0 {
            let lz = num.leading_zeros();
            indices.push(Square::new((63 - lz) as u8));
            num &= !(1 << (63 - lz));
        }
        indices
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
        BitBoard::from_squares(self.to_squares().iter().map(|x| x.rot90_fast32()).collect())
    }

    #[inline]
    pub fn rot180_by_squares(self) -> BitBoard {
        BitBoard::from_squares(self.to_squares().iter().map(|x| x.rot180()).collect())
    }

    #[inline]
    pub fn rot270_by_squares(self) -> BitBoard {
        BitBoard::from_squares(
            self.to_squares()
                .iter()
                .map(|x| x.rot270_fast32())
                .collect(),
        )
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
        BitBoard((self & NOT_H_FILE).0 << 1)
    }

    #[inline]
    pub fn lshift(self) -> BitBoard {
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

impl BitBoard {
    #[inline]
    pub fn test(self, x: X, y: Y) -> bool {
        let pos = BitBoard::set(x, y);
        self & pos == pos
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
    pub fn test_square(&self, square: Square) -> bool {
        let mask = BitBoard::from_square(square);
        self & mask == mask
    }

    #[inline]
    pub fn from_indices(x: usize, y: usize) -> BitBoard {
        BitBoard::set(X::from_index(x), Y::from_index(y))
    }

    #[inline]
    pub fn from_indices_vec(xs: Vec<(usize, usize)>) -> BitBoard {
        let mut b = BitBoard::empty();
        for (rx, ry) in xs {
            b |= BitBoard::from_indices(rx, ry);
        }
        b
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
