use crate::bitboard::{BitBoard, EMPTY};
use crate::coord::{ALL_X, ALL_Y};
use crate::move_tab::LUTEntry;
use crate::orientation::Orientation;
use crate::pieces::{PieceId, PieceList};
use crate::r#move::{Color, Move, MoveVisitor};
use crate::ray::Rays;
use crate::square::*;
use crate::zobrist;

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub hand: PieceList,
    pub move_list: Vec<Move>,
    pub occupied: BitBoard,
    pub owned: BitBoard,
}

#[inline]
fn is_captured(area: BitBoard, group: BitBoard) -> bool {
    let s = BitBoard(0xff00000000000000);
    let w = BitBoard(0x0101010101010101);
    let n = BitBoard(0x00000000000000ff);
    let e = BitBoard(0x8080808080808080);

    !(area.intersects(s) && area.intersects(n) || area.intersects(e) && area.intersects(w))
        && area.get_adjacent_mask().intersects(group)
}

fn find_territory(b: BitBoard, group: BitBoard) -> BitBoard {
    let mut seen = EMPTY;
    let mut territory = BitBoard(0);
    while seen != !EMPTY {
        // Skip forward to the first unset bit
        let i = seen.0.trailing_ones() as u8;
        let pos = Square::new(i);
        debug_assert!(!seen.test_square(pos));

        // Mark this spot preemptively so we don't get stuck
        seen |= BitBoard::from_square(pos);

        // If this board position is empty
        if !b.test_square(pos) {
            // Ge the connected group
            let area = (!b).floodfill8(pos);

            if is_captured(area, group) {
                territory |= area;
            }

            seen |= area;
        }
    }
    territory
}

impl PlayerState {
    #[inline]
    pub fn new() -> PlayerState {
        PlayerState {
            hand: PieceList::full(),
            move_list: Vec::with_capacity(12),
            occupied: EMPTY,
            owned: EMPTY,
        }
    }

    pub fn place(&mut self, m: Move, other: &mut PlayerState) -> bool {
        let piece = m.get_piece();
        self.hand.remove(piece);
        self.move_list.push(m);
        self.occupied |= m.mask();

        let mut capture_flag = false;

        let group = self.occupied.floodfill4(m.to_square());
        let territory = find_territory(self.occupied, group);
        if territory != EMPTY {
            // The new territory is the potential territory minus any existing territory
            let new = (group | territory) & !(self.owned | other.owned);

            // Look for pieces to remove
            other.move_list.retain_mut(|x| {
                // If this isn't a boss and move was on the acquired territory
                if piece != PieceId::Boss && new.intersects(x.mask()) {
                    // Remove from occupied map
                    other.occupied &= !x.mask();

                    // Add piece back to hand
                    other.hand.add(piece);

                    // Remember to rehash the move list
                    capture_flag = true;

                    // Remove from move list
                    return false;
                }

                // Don't remove from move list
                true
            });

            self.owned |= new;

            // Mark as owned all this color's pieces which are within the captured territory.
            self.move_list.iter().for_each(|x| {
                if x.mask().intersects(new) {
                    self.owned |= x.mask();
                }
            });
        } else if self.owned.intersects(m.mask().get_adjacent_mask()) {
            self.owned |= m.mask();
        }

        capture_flag
    }

    #[inline]
    pub fn points(&self) -> u32 {
        ((!self.owned & self.occupied).popcnt() >> 1) + self.occupied.popcnt()
    }

    pub fn moves_str(&self) -> String {
        self.move_list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(Clone)]
pub struct Board {
    pub black: PlayerState,
    pub white: PlayerState,
    pub boss: BitBoard,
    pub rays: Rays,
}

impl Board {
    #[inline]
    fn new() -> Board {
        Board {
            black: PlayerState::new(),
            white: PlayerState::new(),
            boss: EMPTY,
            rays: Rays::new(),
        }
    }

    #[inline]
    pub fn occupied(&self) -> BitBoard {
        self.black.occupied | self.white.occupied
    }

    #[inline]
    pub fn owned(&self) -> BitBoard {
        self.black.owned | self.white.owned
    }

    pub fn print_color_map(&self) {
        println!("   A B C D E F G H");
        for y in ALL_Y {
            print!(" {} ", y.to_index() + 1);
            for x in ALL_X {
                print!(
                    "{:} ",
                    if self.black.occupied.test(x, y) {
                        "X"
                    } else if self.white.occupied.test(x, y) {
                        "O"
                    } else {
                        "."
                    }
                );
            }
            println!("{}", y.to_index() + 1);
        }
        println!("   A B C D E F G H");
    }

    pub fn print_owner_map(&self) {
        println!("   A B C D E F G H");
        for y in ALL_Y {
            print!(" {} ", y.to_index() + 1);
            for x in ALL_X {
                print!(
                    "{:} ",
                    if self.black.owned.test(x, y) {
                        "X"
                    } else if self.white.owned.test(x, y) {
                        "O"
                    } else {
                        "."
                    }
                );
            }
            println!("{}", y.to_index() + 1);
        }
        println!("   A B C D E F G H");
    }

    fn redraw_rays(&mut self) {
        self.rays.clear();
        let mut ms = self.black.move_list.clone();
        ms.extend(self.white.move_list.clone());
        ms.iter().for_each(|m| {
            if m.get_piece() == PieceId::Boss {
                self.draw_boss_rays(*m);
            } else {
                self.rays.draw(m.gaze().to_square(), m.orientation());
            }
        });
    }

    #[inline]
    fn draw_boss_rays(&mut self, m: Move) {
        use Orientation::*;
        let nw = m.position();
        let sw = nw.udown();
        let se = sw.uright();
        let ne = se.uup();

        self.rays.draw(nw, N);
        self.rays.draw(nw, W);
        self.rays.draw(sw, S);
        self.rays.draw(sw, W);
        self.rays.draw(ne, N);
        self.rays.draw(ne, E);
        self.rays.draw(se, S);
        self.rays.draw(se, E);
    }

    #[inline]
    pub fn place(&mut self, color: Color, m: Move) -> bool {
        // x decrement hand
        // x add to move_list
        // x draw rays (extra for boss)
        // x add to color_list
        // x if boss, add to boss
        // x if connection: capture and mark territory
        let capture_flag = match color {
            Color::Black => self.black.place(m, &mut self.white),
            Color::White => self.white.place(m, &mut self.black),
        };
        if m.get_piece() == PieceId::Boss {
            self.boss |= m.mask();
            self.draw_boss_rays(m);
        } else {
            self.rays.draw(m.gaze().to_square(), m.orientation());
        }

        capture_flag
    }

    #[inline]
    fn has_opposite_connection(&self, m: Move, occupied: BitBoard) -> bool {
        (occupied | m.mask()).has_opposite_connection(m.to_square())
    }

    pub fn generate_moves<V>(&self, color: Color, visitor: &mut V)
    where
        V: MoveVisitor,
    {
        let (hand, occupied, owned) = match color {
            Color::Black => (&self.black.hand, self.black.occupied, self.white.owned),
            Color::White => (&self.white.hand, self.white.occupied, self.black.owned),
        };

        let mut hash = PieceList::piece_seen_hash();
        for piece in hand.available() {
            if !hash.seen(piece) {
                hash.add(piece);
                let p = piece.piece_type_id().def();

                for i in p.lut_offset..p.lut_offset + p.moves {
                    let m = Move::new(piece, LUTEntry(i));

                    if self.valid(m, occupied, owned) {
                        visitor.visit(m);
                        if visitor.bailout() {
                            return;
                        };
                    }
                }
            }
        }
    }

    #[inline]
    fn valid(&self, m: Move, occupied: BitBoard, owned: BitBoard) -> bool {
        // x No overlap with other piece
        // x Not face against edge of board
        // x No eye contact with any other neko
        // x Not looking at Boss
        // x Nobi neko cannot touch their own boss with paw
        // x Not forming an opposite board connection

        !m.mask().intersects(self.occupied())
            && !m.mask().intersects(owned)
            && (m.get_piece() == PieceId::Boss
                || (!m
                    .gaze()
                    .intersects(self.rays.get(m.orientation().opposite()))
                    && !self.nobi_paw_overlaps(m.get_piece(), occupied, m)
                    && !self.has_opposite_connection(m, occupied)))
    }

    #[inline]
    fn nobi_paw_overlaps(&self, piece: PieceId, occupied: BitBoard, m: Move) -> bool {
        if piece != PieceId::Nobi {
            return false;
        }

        // The paw is one space to the right on the canonical orientation
        let paw = match m.orientation() {
            Orientation::S => m.gaze().rshift(),
            Orientation::W => m.gaze().dshift(),
            Orientation::N => m.gaze().lshift(),
            Orientation::E => m.gaze().ushift(),
        };

        let boss_adj = (self.boss & occupied).get_adjacent_mask();

        paw.intersects(boss_adj)
    }
}

#[derive(Clone)]
pub struct State {
    pub current: Color,
    pub board: Board,
    pub hash: u64,
    pub capture_flag: bool,
}

impl State {
    #[inline]
    pub fn new() -> State {
        State {
            current: Color::Black,
            board: Board::new(),
            hash: 0,
            capture_flag: false,
        }
    }

    #[inline]
    pub fn place(&mut self, m: Move) {
        self.capture_flag = self.board.place(self.current, m);
    }

    #[inline]
    pub fn hash(&self, m: Move) -> u64 {
        let index = m.get_lut_entry().0;
        let color = self.current as usize & 1;
        // rotational symmetry can't be leveraged because bosses are immobile
        zobrist::HASHES[(index << 1) | color]
    }

    #[inline]
    fn hash_move_list(&self, init: u64, move_list: &[Move]) -> u64 {
        move_list.iter().fold(init, |h, &p| h ^ self.hash(p))
    }

    #[inline]
    pub fn update_hash(&mut self, m: Move) {
        if self.capture_flag {
            // Upon capture, we need to rehash everything since we removed pieces
            let hash_black = self.hash_move_list(0, &self.board.black.move_list);
            self.hash = self.hash_move_list(hash_black, &self.board.white.move_list);
            self.board.redraw_rays();
            self.capture_flag = false;
        } else {
            self.hash ^= self.hash(m)
        }
    }

    pub fn dump(&self) {
        println!("Color map:");
        self.board.print_color_map();
        // println!("\nRays:");
        // println!("{}", self.board.rays);
        println!("Occupied:");
        println!("{}", self.board.occupied());
        println!("Owned:");
        self.board.print_owner_map();

        println!("Black:");
        print!("- hand: ");
        self.board.black.hand.dump();
        println!("- moves: {}", self.board.black.moves_str());

        println!("White:");
        print!("- hand: ");
        self.board.white.hand.dump();
        println!("- moves: {}", self.board.white.moves_str());
    }
}
