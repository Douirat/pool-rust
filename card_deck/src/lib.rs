// ---------------- Suit Enum ----------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value for Suit!"),
        }
    }

    pub fn random(seed: &mut u32) -> Suit {
        *seed = seed.wrapping_mul(1103515245).wrapping_add(12345); // simple LCG
        let x = ((*seed / 65536) % 4 + 1) as u8; // 1..4
        Suit::translate(x)
    }
}

// ---------------- Rank Enum ----------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid value for Rank!"),
        }
    }

    pub fn random(seed: &mut u32) -> Rank {
        *seed = seed.wrapping_mul(1103515245).wrapping_add(12345); // simple LCG
        let x = ((*seed / 65536) % 13 + 1) as u8; // 1..13
        Rank::translate(x)
    }
}

// ---------------- Card Struct ----------------
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// ---------------- winner_card Function ----------------
pub fn winner_card(card: &Card) -> bool {
    card.rank == Rank::Ace && card.suit == Suit::Spade
}
