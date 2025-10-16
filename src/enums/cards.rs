use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardValue{
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardValue::Ace => write!(f, "A"),
            CardValue::King => write!(f, "K"),
            CardValue::Queen => write!(f, "Q"),
            CardValue::Jack => write!(f, "J"),
            CardValue::Ten => write!(f, "10"),
            value => write!(f, "{}", *value as u8 + 2), // For numeric values 2-9
        }
    }
}