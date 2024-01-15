use crate::coord::*;
use crate::error::*;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct Square(u8);

impl Default for Square {
    #[inline]
    fn default() -> Square {
        Square::new(0)
    }
}

impl Square {
    #[inline]
    pub fn new(sq: u8) -> Square {
        Square(sq & 63)
    }

    #[inline]
    pub fn make_square(x: X, y: Y) -> Square {
        Square((y.to_index() as u8) << 3 ^ (x.to_index() as u8))
    }

    #[inline]
    pub fn from_indices(x: usize, y: usize) -> Square {
        Square::make_square(X::from_index(x), Y::from_index(y))
    }

    #[inline]
    pub fn rot90(self) -> Square {
        Square((((self.0 >> 3) | (self.0 << 3)) & 63) ^ 7)
    }

    #[inline]
    pub fn rot90_fast32(self) -> Square {
        Square((((self.0 as u32).wrapping_mul(0x20800000) >> 26) ^ 7) as u8)
    }

    #[inline]
    pub fn rot180(self) -> Square {
        Square(self.0 ^ 63)
    }

    #[inline]
    pub fn rot270(&self) -> Square {
        Square((((self.0 >> 3) | (self.0 << 3)) & 63) ^ 56)
    }

    #[inline]
    pub fn rot270_fast32(&self) -> Square {
        Square((((self.0 as u32).wrapping_mul(0x20800000) >> 26) ^ 56) as u8)
    }

    // rotate 180
    // sq' = sq ^ 63; // 63 - sq;

    #[inline]
    pub fn is_left_edge(&self) -> bool {
        self.get_x() == X::X0
    }

    #[inline]
    pub fn is_right_edge(&self) -> bool {
        self.get_x() == X::X7
    }

    #[inline]
    pub fn is_top_edge(&self) -> bool {
        self.get_y() == Y::Y0
    }

    #[inline]
    pub fn is_bottom_edge(&self) -> bool {
        self.get_y() == Y::Y7
    }

    #[inline]
    pub fn is_edge(&self) -> bool {
        self.is_bottom_edge() || self.is_left_edge() || self.is_top_edge() || self.is_right_edge()
    }

    #[inline]
    pub fn get_x(&self) -> X {
        X::from_index((self.0 & 7) as usize)
    }

    #[inline]
    pub fn get_y(&self) -> Y {
        Y::from_index((self.0 >> 3) as usize)
    }

    #[inline]
    pub fn get_coord(&self) -> (X, Y) {
        (self.get_x(), self.get_y())
    }

    #[inline]
    pub fn left(&self) -> Option<Square> {
        if self.get_x() == X::X0 {
            None
        } else {
            Some(Square::make_square(self.get_x().left(), self.get_y()))
        }
    }

    #[inline]
    pub fn right(&self) -> Option<Square> {
        if self.get_x() == X::X7 {
            None
        } else {
            Some(Square::make_square(self.get_x().right(), self.get_y()))
        }
    }

    #[inline]
    pub fn up(&self) -> Option<Square> {
        if self.get_y() == Y::Y0 {
            None
        } else {
            Some(Square::make_square(self.get_x(), self.get_y().up()))
        }
    }

    #[inline]
    pub fn down(&self) -> Option<Square> {
        if self.get_y() == Y::Y7 {
            None
        } else {
            Some(Square::make_square(self.get_x(), self.get_y().down()))
        }
    }

    #[inline]
    pub fn uleft(&self) -> Square {
        Square::make_square(self.get_x().left(), self.get_y())
    }

    #[inline]
    pub fn uright(&self) -> Square {
        Square::make_square(self.get_x().right(), self.get_y())
    }

    #[inline]
    pub fn uup(&self) -> Square {
        Square::make_square(self.get_x(), self.get_y().up())
    }

    #[inline]
    pub fn udown(&self) -> Square {
        Square::make_square(self.get_x(), self.get_y().down())
    }

    #[inline]
    pub fn to_int(self) -> u8 {
        self.0
    }

    #[inline]
    pub fn to_index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + (self.0 & 7)) as char,
            (b'1' + (self.0 >> 3)) as char
        )
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::InvalidSquare);
        }
        let ch: Vec<char> = s.chars().collect();
        match ch[0] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => {}
            _ => {
                return Err(Error::InvalidSquare);
            }
        }
        match ch[1] {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {}
            _ => {
                return Err(Error::InvalidSquare);
            }
        }
        Ok(Square::make_square(
            X::from_index((ch[1] as usize) - ('1' as usize)),
            Y::from_index((ch[0] as usize) - ('a' as usize)),
        ))
    }
}

pub const ALL_SQUARES: [Square; 64] = [
    Square(0),
    Square(1),
    Square(2),
    Square(3),
    Square(4),
    Square(5),
    Square(6),
    Square(7),
    Square(8),
    Square(9),
    Square(10),
    Square(11),
    Square(12),
    Square(13),
    Square(14),
    Square(15),
    Square(16),
    Square(17),
    Square(18),
    Square(19),
    Square(20),
    Square(21),
    Square(22),
    Square(23),
    Square(24),
    Square(25),
    Square(26),
    Square(27),
    Square(28),
    Square(29),
    Square(30),
    Square(31),
    Square(32),
    Square(33),
    Square(34),
    Square(35),
    Square(36),
    Square(37),
    Square(38),
    Square(39),
    Square(40),
    Square(41),
    Square(42),
    Square(43),
    Square(44),
    Square(45),
    Square(46),
    Square(47),
    Square(48),
    Square(49),
    Square(50),
    Square(51),
    Square(52),
    Square(53),
    Square(54),
    Square(55),
    Square(56),
    Square(57),
    Square(58),
    Square(59),
    Square(60),
    Square(61),
    Square(62),
    Square(63),
];
