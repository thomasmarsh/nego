mod bitboard;
mod coord;
mod error;
mod game;
mod r#move;
mod move_tab;
mod orientation;
mod pieces;
mod ray;
mod square;
mod zobrist;

use crate::bitboard::BitBoard;
use crate::coord::X;
use crate::orientation::{Orientation, ALL_ORIENTATIONS};
use crate::pieces::{PieceTypeId, ALL_PIECE_TYPE_IDS};
use crate::square::{Square, ALL_SQUARES};

use std::fmt;

impl Placement {
    fn shift(&self, x: u8, y: u8) -> Placement {
        Placement {
            orientation: self.orientation,
            piece: self.piece,
            mask: self.mask.rshiftn(x as usize).dshiftn(y as usize),
            gaze: self.gaze.rshiftn(x as usize).dshiftn(y as usize),
            position: Square::make_square(
                self.position.get_x().rightn(x),
                self.position.get_y().downn(y),
            ),
        }
    }

    fn prototype(piece_id: PieceTypeId, orientation: Orientation) -> Placement {
        let piece = piece_id.def();
        let mask = BitBoard(piece.mask[orientation as usize]);
        let gaze = BitBoard::from_square(Square::new(piece.gaze[orientation as usize]));
        assert!(mask | gaze == mask);

        Placement {
            orientation: Orientation::from_index(orientation as u8),
            position: Square::new(0),
            piece: piece_id,
            mask,
            gaze,
        }
    }
}

fn gen_placements(piece_id: PieceTypeId, ps: &mut Vec<Placement>) {
    let piece = piece_id.def();
    let is_boss = piece_id == PieceTypeId::Boss;

    let orientations = if is_boss {
        vec![Orientation::S]
    } else {
        ALL_ORIENTATIONS.to_vec()
    };

    for o in orientations {
        let dim = piece.size_for(o);
        assert!(dim.0 > 0 && dim.0 < 9 && dim.1 > 0 && dim.1 < 9);

        let placement = Placement::prototype(piece_id, o);

        for y in 0..=8 - dim.1 {
            for x in 0..=8 - dim.0 {
                if !(is_boss && (x, y) == (3, 3)) {
                    // no face against edge
                    let shifted = placement.shift(x, y);
                    let face = shifted.gaze.to_square();
                    if !match o {
                        Orientation::S => face.is_bottom_edge(),
                        Orientation::W => face.is_left_edge(),
                        Orientation::N => face.is_top_edge(),
                        Orientation::E => face.is_right_edge(),
                    } {
                        ps.push(shifted);
                    }
                }
            }
        }
    }
}

fn make_placement_tab() {
    println!("pub const MOVE_TAB: [(u8, u8, u64, u64); ??] = [");
    let mut count = 0;
    for piece in ALL_PIECE_TYPE_IDS {
        let mut ps = Vec::new();
        gen_placements(piece, &mut ps);
        println!("    // {:?} moves ({})", piece, ps.len());
        for &m in &ps {
            println!(
                "    ({}, 0x{:02x}, 0x{:016x}, 0x{:016x}), // {}",
                m.orientation as usize,
                m.position.to_int(),
                m.mask.0,
                m.gaze.0,
                m.notation()
            );
            count += 1;
        }
    }
    println!("];");
    println!("count = {}", count);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Placement {
    position: Square,         // 6 bit
    orientation: Orientation, // 2 bit
    piece: PieceTypeId,       // 4 bit
    mask: BitBoard,
    gaze: BitBoard,
}

impl Placement {
    fn notation(&self) -> String {
        let piece = self.piece.notation();

        let pos = format!(
            "{}{}",
            ((b'A' + self.position.get_x().to_int()) as char),
            self.position.get_y().to_int() as u16 + 1
        );

        format!("{} {}{:?}", piece, pos, self.orientation)
    }
}

impl fmt::Display for Placement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Orientation::*;
        for square in ALL_SQUARES {
            let x = if self.gaze.test_square(square) {
                //assert!(self.mask.test_square(square));
                match self.orientation {
                    S => "V",
                    W => "<",
                    N => "^",
                    E => ">",
                }
            } else if self.mask.test_square(square) {
                "O"
            } else {
                "."
            };
            write!(f, "{} ", x)?;
            if square.get_x() == X::X7 {
                writeln!(f)?;
            }
        }
        write!(f, "")
    }
}

fn main() {
    make_placement_tab();
}
