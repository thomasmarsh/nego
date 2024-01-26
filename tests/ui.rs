use nego::{
    core::{
        bitboard::BitBoard,
        move_tab::LUTEntry,
        orientation::Orientation,
        orientation::ALL_ORIENTATIONS,
        pieces::{PieceTypeId, ALL_PIECE_TYPE_IDS},
        r#move,
        square::Square,
    },
    ui::piece::{Part, Parts},
};

#[test]
fn rotate_correctly() {
    for piece_type in ALL_PIECE_TYPE_IDS {
        let parts = Parts::new(piece_type);
        let (w, h) = parts.bounds();
        for dir in ALL_ORIENTATIONS {
            println!("{:?} facing {:?}", piece_type, dir);
            use Orientation::*;
            let expected_bounds = match dir {
                S | N => (w, h),
                E | W => (h, w),
            };
            assert_eq!(parts.facing(dir).bounds(), expected_bounds);
        }
    }
}

// Compare against definitions in core::pieces::PieceType
#[test]
fn match_piece_def() {
    for piece_type in ALL_PIECE_TYPE_IDS {
        let parts = Parts::new(piece_type);
        for dir in ALL_ORIENTATIONS {
            println!("{:?} facing {:?}", piece_type, dir);
            let facing = parts.facing(dir);

            // TEST 1: Assert we match the size
            let bounds = facing.bounds();
            let def = piece_type.def();
            let size = def.size_for(dir);
            assert_eq!(bounds, size);

            // Construct masks from the parts
            let mut part_mask = BitBoard::new(0);
            let mut rect_mask = BitBoard::new(0);
            let mut gaze_mask = BitBoard::new(0);
            for part in facing.0 {
                match part {
                    Part::Circle(x, y) => {
                        part_mask |= BitBoard::from_indices(x as _, y as _);
                    }
                    Part::Face(x, y) => {
                        part_mask |= BitBoard::from_indices(x as _, y as _);
                        gaze_mask |= BitBoard::from_indices(x as _, y as _);
                    }
                    Part::Rect(ox, oy, w, h) => {
                        for y in oy..oy + h {
                            for x in ox..ox + w {
                                rect_mask |= BitBoard::from_indices(x as _, y as _);
                            }
                        }
                    }
                }
            }

            // TEST 2: Assert we match the shape
            let mask = BitBoard::new(def.mask[dir as usize]);
            println!("mask:\n{}", mask);
            println!("part mask:\n{}", part_mask);
            println!("rect mask:\n{}", rect_mask);
            assert_eq!(mask, part_mask);
            if piece_type != PieceTypeId::Mame {
                assert_eq!(mask, rect_mask);
            }

            // TEST 3: Assert we match the gaze origin
            if piece_type != PieceTypeId::Boss {
                let gaze = BitBoard::from_square(Square::new(def.gaze[dir as usize]));
                println!("gaze:\n{}", gaze);
                println!("part mask:\n{}", gaze_mask);
                assert_eq!(gaze, gaze_mask);
            }
        }
    }
}

// Compare against all possible moves
#[test]
fn match_move_def() {
    for piece_type in ALL_PIECE_TYPE_IDS {
        let x = piece_type.def();
        for i in x.lut_offset..x.lut_offset + x.moves {
            let m = r#move::Move::new(piece_type.to_piece_id(), LUTEntry(i));

            let pos = m.position().get_coord();
            let parts = Parts::new(piece_type)
                .facing(m.orientation())
                .translate(pos.0 as _, pos.1 as _);

            // Construct masks from the parts
            let mut part_mask = BitBoard::new(0);
            let mut rect_mask = BitBoard::new(0);
            let mut gaze_mask = BitBoard::new(0);
            for part in parts.0 {
                match part {
                    Part::Circle(x, y) => {
                        assert!(x < 8);
                        assert!(y < 8);
                        part_mask |= BitBoard::from_indices(x as _, y as _);
                    }
                    Part::Face(x, y) => {
                        assert!(x < 8);
                        assert!(y < 8);
                        part_mask |= BitBoard::from_indices(x as _, y as _);
                        gaze_mask |= BitBoard::from_indices(x as _, y as _);
                    }
                    Part::Rect(ox, oy, w, h) => {
                        for y in oy..oy + h {
                            for x in ox..ox + w {
                                assert!(x < 8);
                                assert!(y < 8);
                                rect_mask |= BitBoard::from_indices(x as _, y as _);
                            }
                        }
                    }
                }
            }

            // TEST: Assert we match the expected shape
            println!("mask:\n{}", m.mask());
            println!("part mask:\n{}", part_mask);
            println!("rect mask:\n{}", rect_mask);
            assert_eq!(m.mask(), part_mask);
            if piece_type != PieceTypeId::Mame {
                assert_eq!(m.mask(), rect_mask);
            }
        }
    }
}
