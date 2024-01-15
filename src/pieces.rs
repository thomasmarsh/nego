use crate::orientation::Orientation;

pub struct PieceType {
    pub name_en: &'static str,
    pub name_jp: &'static str,
    pub size: (u8, u8),
    pub qty: u8,
    pub moves: usize,
    pub lut_offset: usize,
    pub mask: [u64; 4],
    pub gaze: [u8; 4],
}

impl PieceType {
    #[inline]
    pub fn size_for(&self, orientation: Orientation) -> (u8, u8) {
        use Orientation::*;
        match orientation {
            S | N => self.size,
            E | W => (self.size.1, self.size.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
#[repr(u16)]
pub enum PieceId {
    Boss,      // BOS
    Mame,      // MAM
    Nobi,      // NOB
    Koubaku1,  // KB1
    Koubaku2,  // KB2
    Koubaku3a, // KB3
    Koubaku3b, // KB3
    Kunoji1a,  // KJ1
    Kunoji1b,  // KJ1
    Kunoji2,   // KJ2
    Kunoji3,   // KJ3
    Kunoji4,   // KJ4
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
#[repr(u16)]
pub enum PieceTypeId {
    Boss,
    Mame,
    Nobi,
    Koubaku1,
    Koubaku2,
    Koubaku3,
    Kunoji1,
    Kunoji2,
    Kunoji3,
    Kunoji4,
}

pub const ALL_PIECES_IDS: [PieceId; 12] = [
    PieceId::Boss,
    PieceId::Mame,
    PieceId::Nobi,
    PieceId::Koubaku1,
    PieceId::Koubaku2,
    PieceId::Koubaku3a,
    PieceId::Koubaku3b,
    PieceId::Kunoji1a,
    PieceId::Kunoji1b,
    PieceId::Kunoji2,
    PieceId::Kunoji3,
    PieceId::Kunoji4,
];

pub const ALL_PIECE_TYPE_IDS: [PieceTypeId; 10] = [
    PieceTypeId::Boss,
    PieceTypeId::Mame,
    PieceTypeId::Nobi,
    PieceTypeId::Koubaku1,
    PieceTypeId::Koubaku2,
    PieceTypeId::Koubaku3,
    PieceTypeId::Kunoji1,
    PieceTypeId::Kunoji2,
    PieceTypeId::Kunoji3,
    PieceTypeId::Kunoji4,
];

impl PieceTypeId {
    #[inline]
    pub fn def(self) -> &'static PieceType {
        match self {
            PieceTypeId::Boss => &BOSS_T,
            PieceTypeId::Mame => &MAME_T,
            PieceTypeId::Nobi => &NOBI_T,
            PieceTypeId::Koubaku1 => &KOUBAKU1_T,
            PieceTypeId::Koubaku2 => &KOUBAKU2_T,
            PieceTypeId::Koubaku3 => &KOUBAKU3_T,
            PieceTypeId::Kunoji1 => &KUNOJI1_T,
            PieceTypeId::Kunoji2 => &KUNOJI2_T,
            PieceTypeId::Kunoji3 => &KUNOJI3_T,
            PieceTypeId::Kunoji4 => &KUNOJI4_T,
        }
    }

    pub fn notation(&self) -> String {
        match self {
            PieceTypeId::Boss => "BOS",
            PieceTypeId::Mame => "MAM",
            PieceTypeId::Nobi => "NOB",
            PieceTypeId::Koubaku1 => "KB1",
            PieceTypeId::Koubaku2 => "KB2",
            PieceTypeId::Koubaku3 => "KB3",
            PieceTypeId::Kunoji1 => "KJ1",
            PieceTypeId::Kunoji2 => "KJ2",
            PieceTypeId::Kunoji3 => "KJ3",
            PieceTypeId::Kunoji4 => "KJ4",
        }
        .to_string()
    }
}

impl PieceId {
    #[inline]
    pub fn from_index(i: u16) -> Option<PieceId> {
        match i {
            0 => Some(PieceId::Boss),
            1 => Some(PieceId::Mame),
            2 => Some(PieceId::Nobi),
            3 => Some(PieceId::Koubaku1),
            4 => Some(PieceId::Koubaku2),
            5 => Some(PieceId::Koubaku3a),
            6 => Some(PieceId::Koubaku3b),
            7 => Some(PieceId::Kunoji1a),
            8 => Some(PieceId::Kunoji1b),
            9 => Some(PieceId::Kunoji2),
            10 => Some(PieceId::Kunoji3),
            11 => Some(PieceId::Kunoji4),
            _ => None,
        }
    }

    #[inline]
    pub fn piece_type_id(self) -> PieceTypeId {
        match self {
            PieceId::Boss => PieceTypeId::Boss,
            PieceId::Mame => PieceTypeId::Mame,
            PieceId::Nobi => PieceTypeId::Nobi,
            PieceId::Koubaku1 => PieceTypeId::Koubaku1,
            PieceId::Koubaku2 => PieceTypeId::Koubaku2,
            PieceId::Koubaku3a => PieceTypeId::Koubaku3,
            PieceId::Koubaku3b => PieceTypeId::Koubaku3,
            PieceId::Kunoji1a => PieceTypeId::Kunoji1,
            PieceId::Kunoji1b => PieceTypeId::Kunoji1,
            PieceId::Kunoji2 => PieceTypeId::Kunoji2,
            PieceId::Kunoji3 => PieceTypeId::Kunoji3,
            PieceId::Kunoji4 => PieceTypeId::Kunoji4,
        }
    }
}

pub struct PieceSeenHash(u16);

impl PieceSeenHash {
    #[inline]
    pub fn mask(piece: PieceId) -> u16 {
        1 << piece.piece_type_id() as u16
    }

    #[inline]
    pub fn seen(&self, piece: PieceId) -> bool {
        let mask = Self::mask(piece);
        mask & self.0 == mask
    }

    #[inline]
    pub fn add(&mut self, piece: PieceId) {
        self.0 |= Self::mask(piece);
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub struct PieceList(u16);

impl PieceList {
    #[inline]
    pub fn full() -> PieceList {
        PieceList(0xfff)
    }

    #[inline]
    pub fn available(self) -> PieceList {
        if self.holding(PieceId::Boss) {
            PieceList(1)
        } else {
            self
        }
    }

    #[inline]
    pub fn empty() -> PieceList {
        PieceList(0)
    }

    #[inline]
    pub fn piece_seen_hash() -> PieceSeenHash {
        PieceSeenHash(0)
    }

    #[inline]
    pub fn holding(&self, piece: PieceId) -> bool {
        let mask = 1 << (piece as usize);
        self.0 & mask == mask
    }

    #[inline]
    pub fn add(&mut self, piece: PieceId) {
        let mask = 1 << (piece as usize);
        self.0 |= mask;
    }

    #[inline]
    pub fn remove(&mut self, piece: PieceId) {
        let mask = 1 << (piece as usize);
        self.0 &= !mask;
    }

    pub fn dump(&self) {
        let mut counts: [u8; 10] = [0; 10];
        for piece in ALL_PIECES_IDS {
            if self.holding(piece) {
                let type_id = piece.piece_type_id();
                counts[type_id as usize] += 1;
            }
        }
        for piece_type in ALL_PIECE_TYPE_IDS {
            print!("{}={} ", piece_type.notation(), counts[piece_type as usize]);
        }
        println!();
    }
}

impl Iterator for PieceList {
    type Item = PieceId;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = self.0.trailing_zeros() as u16;
            self.0 ^= 1 << result;
            PieceId::from_index(result)
        }
    }
}

const BOSS_T: PieceType = PieceType {
    name_en: "Boss",
    name_jp: "ボスネコ",
    size: (2, 2),
    qty: 1,
    moves: 48,
    lut_offset: 0,
    mask: [0x303; 4],
    gaze: [0; 4],
};

const MAME_T: PieceType = PieceType {
    name_en: "Mame",
    name_jp: "マメネコ", // small; miniature; baby; midget; small-scale
    size: (1, 1),
    qty: 1,
    moves: 224,
    lut_offset: BOSS_T.lut_offset + BOSS_T.moves,
    mask: [1; 4],
    gaze: [0; 4],
};

const NOBI_T: PieceType = PieceType {
    name_en: "Nobi",
    name_jp: "ノビネコ", // stretching (one's body, e.g. when waking up)
    size: (4, 1),
    qty: 1,
    moves: 140,
    lut_offset: MAME_T.lut_offset + MAME_T.moves,
    mask: [0xf, 0x1010101, 0xf, 0x1010101],
    gaze: [2, 16, 1, 8],
};

const KOUBAKU1_T: PieceType = PieceType {
    name_en: "Koubaku 1",
    name_jp: "コウバクネコ", // ?? vast; wide; boundless
    size: (2, 1),
    qty: 1,
    moves: 196,
    lut_offset: NOBI_T.lut_offset + NOBI_T.moves,
    mask: [0x3, 0x101, 0x3, 0x101],
    gaze: [0, 0, 1, 8],
};

const KOUBAKU2_T: PieceType = PieceType {
    name_en: "Koubaku 2",
    name_jp: "コウバクネコ",
    size: (2, 1),
    qty: 1,
    moves: 196,
    lut_offset: KOUBAKU1_T.lut_offset + KOUBAKU1_T.moves,
    mask: [0x3, 0x101, 0x3, 0x101],
    gaze: [1, 8, 0, 0],
};

const KOUBAKU3_T: PieceType = PieceType {
    name_en: "Koubaku 3",
    name_jp: "コウバクネコ",
    size: (1, 2),
    qty: 2,
    moves: 192,
    lut_offset: KOUBAKU2_T.lut_offset + KOUBAKU2_T.moves,
    mask: [0x101, 0x3, 0x101, 0x3],
    gaze: [8, 0, 0, 1],
};

const KUNOJI1_T: PieceType = PieceType {
    name_en: "Kunoji 1",
    name_jp: "クノジネコ", // shapped like the letter く
    size: (2, 2),
    qty: 2,
    moves: 168,
    lut_offset: KOUBAKU3_T.lut_offset + KOUBAKU3_T.moves,
    mask: [0x203, 0x302, 0x301, 0x103],
    gaze: [9, 8, 0, 1],
};

const KUNOJI2_T: PieceType = PieceType {
    name_en: "Kunoji 2",
    name_jp: "クノジネコ",
    size: (2, 2),
    qty: 1,
    moves: 168,
    lut_offset: KUNOJI1_T.lut_offset + KUNOJI1_T.moves,
    mask: [0x103, 0x203, 0x302, 0x301],
    gaze: [8, 0, 1, 9],
};

const KUNOJI3_T: PieceType = PieceType {
    name_en: "Kunoji 3",
    name_jp: "クノジネコ",
    size: (2, 2),
    qty: 1,
    moves: 196,
    lut_offset: KUNOJI2_T.lut_offset + KUNOJI2_T.moves,
    mask: [0x203, 0x302, 0x301, 0x103],
    gaze: [0, 1, 9, 8],
};

const KUNOJI4_T: PieceType = PieceType {
    name_en: "Kunoji 4",
    name_jp: "クノジネコ",
    size: (2, 2),
    qty: 1,
    moves: 196,
    lut_offset: KUNOJI3_T.lut_offset + KUNOJI3_T.moves,
    mask: [0x103, 0x203, 0x302, 0x301],
    gaze: [1, 9, 8, 0],
};
