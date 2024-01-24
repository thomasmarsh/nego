use crate::core::{game::Color, orientation::Orientation};

impl Orientation {
    #[inline]
    fn rot90(self) -> Orientation {
        self.right()
    }

    #[inline]
    fn rotation(self) -> f32 {
        use Orientation::*;
        match self {
            S => 0.,
            W => -90.,
            N => 180.,
            E => 90.,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Part {
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
            Part::Circle(x, y) => Part::Circle(height - 2 - y, x),
            Part::Rect(x, y, w, h) => Part::Rect(height - 1 - y - h, x, h, w),
            Part::Face(x, y) => Part::Face(height - 2 - y, x),
        }
    }

    #[inline]
    fn bounds(&self) -> (u8, u8) {
        match *self {
            Part::Circle(x, y) => (x + 1, y + 1),
            Part::Rect(x, y, w, h) => (x + w + 1, y + h + 1),
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

#[derive(Debug)]
pub struct Piece {
    color: Color,
    parts: Vec<Part>,
    offset: (u8, u8),
    facing: Orientation,
}

impl Default for Piece {
    #[inline]
    fn default() -> Self {
        Piece {
            color: Color::Black,
            parts: Vec::new(),
            offset: (0, 0),
            facing: Orientation::S,
        }
    }
}

impl Piece {
    #[inline]
    pub fn bounds(&self) -> (u8, u8) {
        let (mut mx, mut my) = (0, 0);
        self.parts.iter().for_each(|p| {
            let (bx, by) = p.bounds();
            mx = std::cmp::max(mx, bx);
            my = std::cmp::max(my, by);
        });
        (mx, my)
    }

    #[inline]
    pub fn rot90(&self) -> Piece {
        let bounds = self.bounds();
        // TODO: Mame's don't rot270 correctly and end up displaced
        Piece {
            parts: self.parts.iter().map(|p| p.rot90(bounds)).collect(),
            facing: self.facing.rot90(),
            ..*self
        }
    }

    #[inline]
    pub fn facing(&self, facing: Orientation) -> Piece {
        match facing {
            Orientation::S => Piece {
                parts: self.parts.clone(),
                ..*self
            },
            Orientation::W => self.rot90(),
            Orientation::N => self.rot90().rot90(),
            Orientation::E => self.rot90().rot90().rot90(),
        }
    }

    #[inline]
    pub fn translate(&self, dx: i16, dy: i16) -> Piece {
        Piece {
            parts: self.parts.clone(),
            offset: (
                (self.offset.0 as i16 + dx) as u8,
                (self.offset.1 as i16 + dy) as u8,
            ),
            // NOTE: only translate the parts when drawing
            ..*self
        }
    }

    #[inline]
    pub fn draw(&self) {
        use crate::ui::draw;

        let c = self.color;
        for part in &self.parts {
            match part.translate(self.offset.0, self.offset.1) {
                Part::Circle(x, y) => draw::circle(c, x, y),
                Part::Face(x, y) => draw::triangle(c, x, y, self.facing.rotation()),
                Part::Rect(x, y, w, h) => draw::rect(c, x, y, w, h),
            }
        }
    }
}

#[inline]
fn mk_piece(color: Color) -> Piece {
    Piece {
        color,
        ..Piece::default()
    }
}

#[inline]
pub fn mk_boss(color: Color) -> Piece {
    Piece {
        parts: vec![
            Part::Circle(0, 0),
            Part::Circle(1, 0),
            Part::Circle(0, 1),
            Part::Circle(1, 1),
            Part::Rect(0, 0, 2, 2),
        ],
        ..mk_piece(color)
    }
}

#[inline]
pub fn mk_mame(color: Color) -> Piece {
    Piece {
        parts: vec![Part::Face(0, 0)],
        ..mk_piece(color)
    }
}

#[inline]
pub fn mk_nobi(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![
            Circle(0, 0),
            Circle(1, 0),
            Face(2, 0),
            Circle(3, 0),
            Rect(0, 0, 4, 1),
        ],
        ..mk_piece(color)
    }
}

#[inline]
pub fn mk_koubaku1(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![Face(0, 0), Circle(1, 0), Rect(0, 0, 2, 1)],
        ..mk_piece(color)
    }
}

#[inline]
pub fn mk_koubaku2(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![Circle(0, 0), Face(1, 0), Rect(0, 0, 2, 1)],
        ..mk_piece(color)
    }
}

#[inline]
pub fn mk_koubaku3(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![Circle(0, 0), Face(0, 1), Rect(0, 0, 1, 2)],
        ..mk_piece(color)
    }
}

#[inline]
pub fn mk_kunoji1(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![
            Circle(0, 0),
            Circle(1, 0),
            Face(1, 1),
            Rect(0, 0, 2, 1),
            Rect(1, 0, 1, 2),
        ],
        ..mk_piece(color)
    }
}

// 203 = L
#[inline]
pub fn mk_kunoji2(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![
            Circle(1, 0),
            Circle(0, 0),
            Face(0, 1),
            Rect(0, 0, 1, 2),
            Rect(0, 0, 2, 1),
        ],
        ..mk_piece(color)
    }
}

// 103 = ⅃
#[inline]
pub fn mk_kunoji3(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![
            Face(0, 0),
            Circle(1, 0),
            Circle(1, 1),
            Rect(0, 0, 2, 1),
            Rect(1, 0, 1, 2),
        ],
        ..mk_piece(color)
    }
}

// 203 = L
#[inline]
pub fn mk_kunoji4(color: Color) -> Piece {
    use Part::*;
    Piece {
        parts: vec![
            Face(1, 0),
            Circle(0, 0),
            Circle(0, 1),
            Rect(0, 0, 1, 2),
            Rect(0, 0, 2, 1),
        ],
        ..mk_piece(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserve_origin() {
        use Orientation::*;
        let c = Color::Black;
        for (label, piece) in [
            ("boss", mk_boss(c)),
            ("mame", mk_mame(c)),
            ("nobi", mk_nobi(c)),
            ("koubaku1", mk_koubaku1(c)),
            ("koubaku2", mk_koubaku2(c)),
            ("koubaku3", mk_koubaku3(c)),
            ("kunoji1", mk_kunoji1(c)),
            ("kunoji2", mk_kunoji2(c)),
            ("kunoji3", mk_kunoji3(c)),
            ("kunoji4", mk_kunoji4(c)),
        ] {
            for y in 0..8 {
                for x in 0..8 {
                    for dir in [S, W, N, E] {
                        println!("{} at ({}, {}) {:?}", label, x, y, dir);
                        assert_eq!(piece.translate(x, y).facing(dir).offset, (x as _, y as _))
                    }
                }
            }
        }
    }
}
