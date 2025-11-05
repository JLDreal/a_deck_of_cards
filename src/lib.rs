/// A deck of cards library.
///
/// This library provides a deck of cards with various operations such as shuffling, taking cards, and renewing the deck.
///
/// # Examples
///
/// ```
/// use a_deck_of_cards::deck::Deck;
///
/// let deck = Deck::new();
/// assert_eq!(deck.cards.len(), 52);
/// ```
pub mod deck;

/// Enums module containing card values and suits.
///
/// This module provides enums for card values and suits used in the deck of cards.
///
/// # Examples
///
/// ```
/// use a_deck_of_cards::enums::cards::CardValue;
/// use a_deck_of_cards::enums::suits::Suit;
///
/// let value = CardValue::Ace;
/// let suit = Suit::Hearts;
/// ```
pub mod enums;
