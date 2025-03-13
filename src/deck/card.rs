use crate::enums::cards::CardValue;
use crate::enums::suits::Suit;
use std::sync::atomic::Ordering;
use std::cmp::Ordering as OtherPrdering;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit, 
    pub value: CardValue
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

