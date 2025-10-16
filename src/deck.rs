pub use crate::enums::{cards::CardValue, suits::Suit};

use rand::{thread_rng, Rng}; // Import from the top-level `enums` module

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub value: CardValue,
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[derive(Debug, PartialEq)]
pub enum DeckError {
    CardNotFound,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for suit in Suit::all() {
            for value in CardValue::all() {
                cards.push(Card { suit, value });
            }
        }

        Deck { cards }
    }
    pub fn remove_at(&mut self, index: usize) -> Result<Card, DeckError> {
        if index >= self.cards.len() {
            Err(DeckError::CardNotFound)
        } else {
            Ok(self.cards.remove(index))
        }
    }
    pub fn take_top(&mut self) -> Result<Card, DeckError> {
        self.remove_at(0)
    }
    pub fn renew(&mut self) {
        self.cards = Deck::new().cards;
        self.shuffle();
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        for i in 0..self.cards.len() {
            let j = rng.gen_range(0..=i);
            self.cards.swap(i, j);
        }
    }
}
impl Default for Deck {
    fn default() -> Self {
        Deck {
            cards: Vec::with_capacity(52),
        }
    }
}
