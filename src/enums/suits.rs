 // Declare the `card` module (located at src/deck/card.rs)
pub use crate::enums::cards::CardValue; // Import from the top-level `enums` module
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}
// src/enums/suits.rs
impl Suit {
    pub fn all() -> [Self; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }
}


impl CardValue {
    pub fn all() -> [Self; 13] {
        [
            CardValue::Two,
            CardValue::Three,
            CardValue::Four,
            CardValue::Five,
            CardValue::Six,
            CardValue::Seven,
            CardValue::Eight,
            CardValue::Nine,
            CardValue::Ten,
            CardValue::Jack,
            CardValue::Queen,
            CardValue::King,
            CardValue::Ace,
        ]
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}