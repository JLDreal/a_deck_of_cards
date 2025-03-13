mod card; // Declare the `card` module (located at src/deck/card.rs)
pub use crate::enums::{cards::CardValue, suits::Suit}; // Import from the top-level `enums` module

pub use self::card::Card; // Re-export `Card` from the `card` module

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
                cards.push(Card {
                    suit,
                    value,
                });
            }
        }
        
        Deck { cards }
    }
    fn remove_at(&mut self, index: usize) -> Result<Card, DeckError> {
        if index >= self.cards.len() {
            Err(DeckError::CardNotFound)
        } else {
            Ok(self.cards.remove(index))
        }
    }
    pub fn take_top(&mut self) -> Result<Card, DeckError> {
        self.remove_at(0)
    }
}