use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

/* ---------------- FromStr ---------------- */

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_part, rh_part) = s.split_at(s.len() - 1);

        let antigen = match antigen_part {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => return Err(()),
        };

        let rh_factor = match rh_part {
            "+" => RhFactor::Positive,
            "-" => RhFactor::Negative,
            _ => return Err(()),
        };

        Ok(BloodType { antigen, rh_factor })
    }
}

/* ---------------- Debug ---------------- */

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen, rh)
    }
}

/* ---------------- BloodType logic ---------------- */

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        // Rh compatibility
        let rh_ok = match (self.rh_factor, other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };

        // Antigen compatibility
        let antigen_ok = match (self.antigen, other.antigen) {
            (_, Antigen::O) => true,
            (Antigen::A, Antigen::A) => true,
            (Antigen::B, Antigen::B) => true,
            (Antigen::AB, _) => true,
            _ => false,
        };

        rh_ok && antigen_ok
    }

    pub fn donors(self) -> Vec<Self> {
        all_blood_types()
            .into_iter()
            .filter(|&bt| self.can_receive_from(bt))
            .collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        all_blood_types()
            .into_iter()
            .filter(|&bt| bt.can_receive_from(self))
            .collect()
    }
}

/* ---------------- Helper ---------------- */

fn all_blood_types() -> Vec<BloodType> {
    let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
    let rhs = [RhFactor::Positive, RhFactor::Negative];

    let mut res = Vec::new();
    for &a in &antigens {
        for &r in &rhs {
            res.push(BloodType {
                antigen: a,
                rh_factor: r,
            });
        }
    }
    res
}
