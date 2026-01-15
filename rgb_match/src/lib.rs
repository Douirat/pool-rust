#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let Color { r, g, b, a } = self;

        match (r, g, b, a) {
            // r <-> g
            (f, s, b, a) if f == first && s == second => Color { r: s, g: f, b, a },
            (s, f, b, a) if s == first && f == second => Color { r: f, g: s, b, a },

            // r <-> b
            (f, g, s, a) if f == first && s == second => Color { r: s, g, b: f, a },
            (s, g, f, a) if s == first && f == second => Color { r: f, g, b: s, a },

            // r <-> a
            (f, g, b, s) if f == first && s == second => Color { r: s, g, b, a: f },
            (s, g, b, f) if s == first && f == second => Color { r: f, g, b, a: s },

            // g <-> b
            (r, f, s, a) if f == first && s == second => Color { r, g: s, b: f, a },
            (r, s, f, a) if s == first && f == second => Color { r, g: f, b: s, a },

            // g <-> a
            (r, f, b, s) if f == first && s == second => Color { r, g: s, b, a: f },
            (r, s, b, f) if s == first && f == second => Color { r, g: f, b, a: s },

            // b <-> a
            (r, g, f, s) if f == first && s == second => Color { r, g, b: s, a: f },
            (r, g, s, f) if s == first && f == second => Color { r, g, b: f, a: s },

            // nothing matched → unchanged
            _ => self,
        }
    }
}