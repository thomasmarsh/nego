use crate::bitboard::{BitBoard, EMPTY};
use crate::coord::X;
use crate::orientation::{Orientation, ALL_ORIENTATIONS};
use crate::square::{Square, ALL_SQUARES};

use std::fmt;

static mut RAY_LUT: [[BitBoard; 64]; 4] = [[BitBoard(0); 64]; 4];

#[derive(Clone)]
pub struct Rays {
    pub s: BitBoard,
    pub w: BitBoard,
    pub n: BitBoard,
    pub e: BitBoard,
}

impl Rays {
    pub fn new() -> Rays {
        Rays {
            s: EMPTY,
            w: EMPTY,
            n: EMPTY,
            e: EMPTY,
        }
    }

    pub fn get(&self, orientation: Orientation) -> BitBoard {
        use Orientation::*;
        match orientation {
            S => self.s,
            W => self.w,
            N => self.n,
            E => self.e,
        }
    }

    pub fn clear(&mut self) {
        self.s = EMPTY;
        self.w = EMPTY;
        self.n = EMPTY;
        self.e = EMPTY;
    }

    fn draw_raw(&mut self, initial: Square, orientation: Orientation) {
        use Orientation::*;
        let mut next = Some(initial);
        while let Some(cur) = next {
            self.set(cur, orientation);
            match orientation {
                S => next = cur.down(),
                W => next = cur.left(),
                N => next = cur.up(),
                E => next = cur.right(),
            }
        }
    }

    pub fn lookup(square: Square, orientation: Orientation) -> BitBoard {
        unsafe { RAY_LUT[orientation as usize][square.to_index()] }
    }

    pub fn build_lut() {
        use Orientation::*;
        for o in ALL_ORIENTATIONS {
            for square in ALL_SQUARES {
                let mut r = Rays::new();
                r.draw_raw(square, o);
                let &r = match o {
                    S => &r.s,
                    W => &r.w,
                    N => &r.n,
                    E => &r.e,
                };
                unsafe {
                    RAY_LUT[o as usize][square.to_index()] = r;
                }
            }
        }
    }

    fn set(&mut self, square: Square, orientation: Orientation) {
        use Orientation::*;
        let pos = BitBoard::from_square(square);
        match orientation {
            S => self.s |= pos,
            W => self.w |= pos,
            N => self.n |= pos,
            E => self.e |= pos,
        }
    }

    #[inline]
    pub fn draw(&mut self, square: Square, orientation: Orientation) {
        use Orientation::*;
        let r = Self::lookup(square, orientation);

        match orientation {
            S => self.s |= r,
            W => self.w |= r,
            N => self.n |= r,
            E => self.e |= r,
        };
    }

    #[inline]
    fn is_set(&self, square: Square, orientation: Orientation) -> bool {
        self.get(orientation).test_square(square)
    }

    fn debug(&self) {
        for o in ALL_ORIENTATIONS {
            println!("{:?}:", o);
            println!("{}", self.get(o));
        }
    }
}

// TODO!
impl fmt::Display for Rays {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Orientation::*;
        write!(f, "  A B C D E F G H\n")?;
        for square in ALL_SQUARES {
            if square.get_x() == X::X1 {
                write!(f, " {} ", square.get_y().to_index() + 1)?;
            }
            let mut os: u8 = 0;
            for o in ALL_ORIENTATIONS {
                let is_set = self.is_set(square, o);
                if is_set {
                    os |= 1 << o as usize;
                }
            }
            let c = match os.count_ones() {
                0 => ".",
                1 => match Orientation::from_index(os.trailing_zeros() as u8) {
                    S => "v",
                    W => "<",
                    N => "^",
                    E => ">",
                },
                _ => "x",
            };
            write!(f, "{} ", c)?;
            if square.get_x() == X::X7 {
                write!(f, " {}\n", square.get_y().to_index() + 1)?;
            }
        }
        write!(f, "  A B C D E F G H\n")?;
        write!(f, "")
    }
}
