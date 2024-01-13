use crate::bitboard::{BitBoard, EMPTY};
use crate::coord::{ALL_X, ALL_Y};
use crate::move_tab::LUTEntry;
use crate::orientation::Orientation;
use crate::pieces::{PieceId, PieceList, ALL_PIECES_IDS};
use crate::r#move::{Color, Move, MoveVisitor};
use crate::ray::Rays;
use crate::square::*;
use crate::zobrist;

use log::{error, trace};

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub hand: PieceList,
    pub move_list: Vec<Move>,
    pub occupied: BitBoard,
    pub owned: BitBoard,
}

fn find_opposite_corner(b: BitBoard, m: &Move) -> Option<BitBoard> {
    let se = BitBoard(0xff80808080808080);
    let sw = BitBoard(0xff01010101010101);
    let ne = BitBoard(0x80808080808080ff);
    let nw = BitBoard(0x01010101010101ff);
    let s = BitBoard(0xff00000000000000);
    let w = BitBoard(0x0101010101010101);
    let n = BitBoard(0x00000000000000ff);
    let e = BitBoard(0x8080808080808080);
    let has_walls = |x: BitBoard, y: BitBoard| x.intersects(b) && y.intersects(b);

    match (
        b.connected(s, e) as u8,
        b.connected(s, w) as u8,
        b.connected(n, e) as u8,
        b.connected(n, w) as u8,
    ) {
        (1, 0, 0, 0) => Some(nw),
        (0, 1, 0, 0) => Some(ne),
        (0, 0, 1, 0) => Some(sw),
        (0, 0, 0, 1) => Some(se),
        (0, 0, 0, 0) => None,
        _ => {
            error!("move: {:?}\n{}", m, m.mask());
            error!("has se walls:{}\n{}", has_walls(s, e), b & se);
            error!("b:\n{}", b);
            error!("has sw walls:{}\n{}", has_walls(s, w), b & sw);
            error!("has ne walls:{}\n{}", has_walls(n, e), b & ne);
            error!("has nw walls:{}\n{}", has_walls(n, w), b & nw);
            error!("multiple corners!");
            panic!();
        }
    }
}

pub fn find_territory(b: BitBoard, group: BitBoard, opposite: BitBoard) -> BitBoard {
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
            let area = b.floodfill(pos);

            // Check that this is wall adjacent, group adjacent, and not an opposite wall
            if area.touches_wall() && !area.intersects(opposite) && area.is_adjacent(group) {
                territory |= area;
            }
            seen |= area;
        }
    }
    territory
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            hand: PieceList::full(),
            move_list: Vec::with_capacity(12),
            occupied: EMPTY,
            owned: EMPTY,
        }
    }

    pub fn place(&mut self, m: &Move, other: &mut PlayerState) -> bool {
        self.hand.remove(m.piece);
        self.move_list.push(*m);
        self.occupied |= m.mask();
        if self.owned.intersects(m.mask().get_adjacent_mask()) {
            self.owned |= m.mask();
        }

        let mut capture_flag = false;

        let group = self.occupied.find_group(m.mask().to_square());
        // This is wrong. A territory can be captured by surround spaces along "up to 2" walls.
        // (Keep in mind that no opposite-board connections are allowed.)
        // So, maybe the correct rules are that the potential territory is:
        // - surrounded by group (if zero wall loop captures allowed)
        // - surrounded by 1 or 2 walls and the group
        if let Some(opposite) = find_opposite_corner(group, m) {
            let territory = find_territory(self.occupied, group, opposite);
            if territory != EMPTY {
                // The new territory is the potential territory minus any existing territory
                let new = (group | territory) ^ (self.owned | other.owned);

                // Look for pieces to remove
                other.move_list.retain_mut(|x| {
                    // If this isn't a boss and move was on the acquired territory
                    if x.piece != PieceId::Boss && new.intersects(x.mask()) {
                        // Remove from occupied map
                        other.occupied &= !x.mask();
                        // Add piece back to hand
                        other.hand.add(x.piece);
                        capture_flag = true;
                        return false;
                    }
                    true
                });
                self.owned |= new;
            }
        }
        capture_flag
    }

    pub fn points(&self) -> u32 {
        self.occupied.popcnt()
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

    fn redraw_rays(&mut self) {
        self.rays.clear();
        let mut ms = self.black.move_list.clone();
        ms.extend(self.white.move_list.clone());
        ms.iter().for_each(|m| {
            if m.piece == PieceId::Boss {
                self.draw_boss_rays(m);
            } else {
                self.rays.draw(m.gaze().to_square(), m.orientation());
            }
        });
    }

    #[inline]
    fn draw_boss_rays(&mut self, m: &Move) {
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
    pub fn place(&mut self, m: &Move) -> bool {
        // x decrement hand
        // x add to move_list
        // x draw rays (extra for boss)
        // x add to color_list
        // x if boss, add to boss
        // x if connection: capture and mark territory
        let capture_flag = match m.color {
            Color::Black => self.black.place(m, &mut self.white),
            Color::White => self.white.place(m, &mut self.black),
        };
        if m.piece == PieceId::Boss {
            self.boss |= m.mask();
            self.draw_boss_rays(m);
        } else {
            self.rays.draw(m.gaze().to_square(), m.orientation());
        }

        capture_flag
    }

    #[inline]
    fn has_opposite_connection(&self, m: &Move, occupied: BitBoard) -> bool {
        (occupied | m.mask()).has_opposite_connection(m.mask().to_square())
    }

    pub fn generate_moves<V>(&self, color: Color, visitor: &mut V)
    where
        V: MoveVisitor,
    {
        //println!("## generate_moves({:?})", color);
        let (hand, occupied) = match color {
            Color::Black => (&self.black.hand, self.black.occupied),
            Color::White => (&self.white.hand, self.white.occupied),
        };

        if hand.holding(PieceId::Boss) {
            self.piece_moves(PieceId::Boss, color, occupied, visitor);
        } else {
            let mut hash = PieceList::piece_seen_hash();
            // trailingzeros here..
            for piece in ALL_PIECES_IDS {
                if !hash.seen(piece) && hand.holding(piece) {
                    hash.add(piece);
                    self.piece_moves(piece, color, occupied, visitor);
                    if visitor.bailout() {
                        return;
                    };
                }
            }
        }
    }

    fn piece_moves<V>(&self, piece: PieceId, color: Color, occupied: BitBoard, visitor: &mut V)
    where
        V: MoveVisitor,
    {
        let p = piece.piece_type_id().def();

        for i in p.lut_offset..p.lut_offset + p.moves {
            let m = Move {
                color,
                piece,
                entry: LUTEntry(i),
            };

            // x No overlap with other piece
            // x Not face against edge of board
            // x No eye contact with any other neko
            // x Not looking at Boss
            // x Nobi neko cannot touch their own boss with paw
            // x Not forming an opposite board connection

            if !m.mask().intersects(self.occupied())
                && (piece == PieceId::Boss
                    || (!m
                        .gaze()
                        .intersects(self.rays.get(m.orientation().opposite()))
                        && !self.nobi_paw_overlaps(piece, occupied, &m)
                        && !self.has_opposite_connection(&m, occupied)))
            {
                visitor.visit(m);
                if visitor.bailout() {
                    return;
                };
            }
        }
    }

    #[inline]
    fn nobi_paw_overlaps(&self, piece: PieceId, occupied: BitBoard, m: &Move) -> bool {
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
    pub fn new() -> State {
        State {
            current: Color::Black,
            board: Board::new(),
            hash: 0,
            capture_flag: false,
        }
    }

    pub fn place(&mut self, m: &Move) {
        self.capture_flag = self.board.place(m);
    }

    #[inline]
    pub fn hash(&self, m: &Move) -> u64 {
        let index = m.entry.0;
        let color = self.current as usize & 1;
        // rotational symmetry can't be leveraged because bosses are immobile
        zobrist::HASHES[(index << 1) | color]
    }

    #[inline]
    pub fn update_hash(&mut self, m: &Move) {
        if self.capture_flag {
            // Upon capture, we need to rehash everything since we removed pieces
            let hash_black = self
                .board
                .black
                .move_list
                .iter()
                .fold(0, |h, &p| h ^ self.hash(&p));
            self.hash = self
                .board
                .white
                .move_list
                .iter()
                .fold(hash_black, |h, &p| h ^ self.hash(&p));
            self.board.redraw_rays();
            self.capture_flag = false;
        } else {
            self.hash ^= self.hash(m)
        }
    }

    pub fn dump(&self) {
        println!("Color map:");
        self.board.print_color_map();
        println!("\nRays:");
        println!("{}", self.board.rays);
        println!("Boss:");
        println!("{}", self.board.boss);
        println!("Occupied:");
        println!("{}", self.board.occupied());
        println!("Owned:");
        println!("{}", self.board.owned());

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
