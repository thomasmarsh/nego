use crate::core::{game::Color, orientation::Orientation, pieces::PieceTypeId, r#move};

#[derive(Copy, Clone, Debug)]
pub enum Part {
    Circle(u8, u8),
    Rect(u8, u8, u8, u8),
    Face(u8, u8),
}

impl Part {
    #[inline]
    fn rot90(&self, bounds: (u8, u8)) -> Part {
        if bounds == (1, 1) {
            return *self;
        }
        let (_width, height) = bounds;
        assert!(height > 0 && _width > 0);
        match *self {
            Part::Circle(x, y) => Part::Circle(height - 1 - y, x),
            Part::Rect(x, y, w, h) => Part::Rect(height - y - h, x, h, w),
            Part::Face(x, y) => Part::Face(height - 1 - y, x),
        }
    }

    #[inline]
    fn bounds(&self) -> (u8, u8) {
        match *self {
            Part::Circle(x, y) => (x + 1, y + 1),
            Part::Rect(x, y, w, h) => (x + w, y + h),
            Part::Face(x, y) => (x + 1, y + 1),
        }
    }

    #[inline]
    fn translate(&self, dx: u8, dy: u8) -> Part {
        match *self {
            Part::Circle(x, y) => Part::Circle(dx + x, dy + y),
            Part::Rect(x, y, w, h) => Part::Rect(dx + x, dy + y, w, h),
            Part::Face(x, y) => Part::Face(dx + x, dy + y),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Parts(pub Vec<Part>);

impl Parts {
    pub fn new(piece: PieceTypeId) -> Parts {
        Parts(parts::for_piece(piece))
    }

    #[inline]
    pub fn bounds(&self) -> (u8, u8) {
        let (mut mx, mut my) = (1, 1);
        self.0.iter().for_each(|p| {
            let (bx, by) = p.bounds();
            mx = std::cmp::max(mx, bx);
            my = std::cmp::max(my, by);
        });
        (mx, my)
    }

    #[inline]
    pub fn rot90(&self) -> Parts {
        let bounds = self.bounds();
        Parts(self.0.iter().map(|p| p.rot90(bounds)).collect())
    }

    #[inline]
    pub fn facing(&self, facing: Orientation) -> Parts {
        match facing {
            Orientation::S => self.clone(),
            Orientation::W => self.rot90(),
            Orientation::N => self.rot90().rot90(),
            Orientation::E => self.rot90().rot90().rot90(),
        }
    }

    #[inline]
    pub fn translate(&self, dx: u8, dy: u8) -> Parts {
        Parts(self.0.iter().map(|part| part.translate(dx, dy)).collect())
    }

    #[inline]
    pub fn draw(&self, c: Color, m: r#move::Move) {
        use crate::ui::draw;
        let coord = m.position().get_coord();
        let (dx, dy) = (coord.0 as u8, coord.1 as u8);

        use Orientation::*;
        let rotation = match m.orientation() {
            S => 0.,
            W => -90.,
            N => 180.,
            E => 90.,
        };
        self.facing(m.orientation())
            .0
            .iter()
            .for_each(|part| match *part {
                Part::Circle(x, y) => draw::circle(c, dx + x, dy + y),
                Part::Face(x, y) => draw::triangle(c, dx + x, dy + y, rotation),
                Part::Rect(x, y, w, h) => draw::rect(c, dx + x, dy + y, w, h),
            });
    }
}

pub mod parts {
    use super::{Part, Part::*};
    use crate::core::pieces::PieceTypeId;

    pub fn for_piece(piece: PieceTypeId) -> Vec<Part> {
        PARTS[piece as usize].to_vec()
    }

    const PARTS: [&[Part]; 10] = [
        // Boss
        &[
            Part::Circle(0, 0),
            Part::Circle(1, 0),
            Part::Circle(0, 1),
            Part::Circle(1, 1),
            Part::Rect(0, 0, 2, 2),
        ],
        // Mame
        &[Part::Face(0, 0)],
        // Nobi
        &[
            Circle(0, 0),
            Circle(1, 0),
            Face(2, 0),
            Circle(3, 0),
            Rect(0, 0, 4, 1),
        ],
        // Koubaku1
        &[Face(0, 0), Circle(1, 0), Rect(0, 0, 2, 1)],
        // Koubaku2
        &[Circle(0, 0), Face(1, 0), Rect(0, 0, 2, 1)],
        // Koubaku3
        &[Circle(0, 0), Face(0, 1), Rect(0, 0, 1, 2)],
        // Kunoji1
        &[
            Circle(0, 0),
            Circle(1, 0),
            Face(1, 1),
            Rect(0, 0, 2, 1),
            Rect(1, 0, 1, 2),
        ],
        // Kunoji2
        &[
            Circle(1, 0),
            Circle(0, 0),
            Face(0, 1),
            Rect(0, 0, 1, 2),
            Rect(0, 0, 2, 1),
        ],
        // Kunoji3
        &[
            Face(0, 0),
            Circle(1, 0),
            Circle(1, 1),
            Rect(0, 0, 2, 1),
            Rect(1, 0, 1, 2),
        ],
        // Kunoji4
        &[
            Face(1, 0),
            Circle(0, 0),
            Circle(0, 1),
            Rect(0, 0, 1, 2),
            Rect(0, 0, 2, 1),
        ],
    ];
}
