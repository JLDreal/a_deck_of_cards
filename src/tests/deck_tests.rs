use crate::deck::{Deck, DeckError};
use crate::enums::cards::CardValue;
use crate::enums::suits::Suit;

#[cfg(test)]
mod tests {
    use crate::deck::Card;

    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
        // Check that all suits and values are present
        for suit in Suit::all() {
            for value in CardValue::all() {
                assert!(deck.cards.contains(&Card { suit, value }));
            }
        }
    }

    #[test]
    fn test_take_top() {
        let mut deck = Deck::new();
        let card = deck.take_top().unwrap();
        assert_eq!(deck.cards.len(), 51);
        assert_eq!(
            card,
            Card {
                suit: Suit::Hearts,
                value: CardValue::Two
            }
        );
    }

    #[test]
    fn test_take_top_error() {
        let mut deck = Deck::new();
        for _ in 0..52 {
            deck.take_top().unwrap();
        }
        assert_eq!(deck.take_top(), Err(DeckError::CardNotFound));
    }

    #[test]
    fn test_shuffle() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();
        deck1.shuffle();
        // Very unlikely that two shuffled decks would be identical
        assert_ne!(deck1.cards, deck2.cards);
    }

    #[test]
    fn test_renew() {
        let mut deck = Deck::new();
        let original_cards = deck.cards.clone();
        deck.take_top().unwrap();
        deck.renew();
        assert_ne!(deck.cards, original_cards);
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_remove_at_error() {
        let mut deck = Deck::new();
        assert_eq!(deck.remove_at(52), Err(DeckError::CardNotFound));
    }

    #[test]
    fn test_remove_at_success() {
        let mut deck = Deck::new();
        let card = deck.remove_at(0).unwrap();
        assert_eq!(
            card,
            Card {
                suit: Suit::Hearts,
                value: CardValue::Two
            }
        );
        assert_eq!(deck.cards.len(), 51);
    }
}
