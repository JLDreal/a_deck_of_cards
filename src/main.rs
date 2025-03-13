mod deck; // Declare the `deck` module (located at src/deck.rs)
mod enums;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::deck::Card;
use crate::enums::{cards::CardValue, suits::Suit};

fn main() {
    let ace_of_spades = Card { value: CardValue::Ace, suit: Suit::Spades };
    let ace_of_hearts = Card { value: CardValue::Ace, suit: Suit::Hearts };
    let king_of_spades = Card { value: CardValue::King, suit: Suit::Spades };

    assert!(ace_of_spades > ace_of_hearts);  // Same value, higher suit
    assert!(ace_of_spades > king_of_spades); // Higher value
    assert!(ace_of_hearts == ace_of_hearts); // Same card
    let mut deck = crate::deck::Deck::new();
    println!("Generated deck with {} cards", deck.cards.len());

    let mut rng = thread_rng();
    deck.cards.shuffle(&mut rng);
    // Print first 5 cards
    for card in deck.cards.iter() {
        println!("{}", card);
    }

    println!("{}",deck.cards.len());
    let c = deck.take_top().unwrap();
    println!("{}",deck.cards.len());
    println!("{}", c)
    
}
