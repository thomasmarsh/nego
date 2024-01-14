use crate::bitboard::{BitBoard, EMPTY};
use crate::coord::{ALL_X, ALL_Y};
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

impl fmt::Display for Rays {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        s.push_str("   A B C D E F G H\n");
        for y in ALL_Y {
            s.push_str(format!(" {} ", y.to_index() + 1).as_str());
            for x in ALL_X {
                let square = Square::make_square(x, y);
                let os: usize = ALL_ORIENTATIONS.iter().fold(0, |acc, o| {
                    let is_set = self.is_set(square, *o);
                    if is_set {
                        acc | (1 << *o as usize)
                    } else {
                        acc
                    }
                });
                let c = match os.count_ones() {
                    0 => ".",
                    1 => match Orientation::from_index(os.trailing_zeros() as u8) {
                        Orientation::S => "v",
                        Orientation::W => "<",
                        Orientation::N => "^",
                        Orientation::E => ">",
                    },
                    _ => "x",
                };
                s.push_str(format!("{} ", c).as_str());
            }
            s.push_str(format!("{}\n", y.to_index() + 1).as_str());
        }
        s.push_str("   A B C D E F G H\n");
        write!(f, "{}", s)
    }
}
