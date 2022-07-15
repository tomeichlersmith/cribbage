use std::{
    fmt,
    str,
};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, EnumIter)]
pub enum Suit {
    Heart,
    Spade,
    Diamond,
    Club
}

impl FromStr for Suit {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H"|"h" => Ok(Suit::Heart),
            "S"|"s" => Ok(Suit::Spade),
            "D"|"d" => Ok(Suit::Diamond),
            "C"|"c" => Ok(Suit::Club),
            _ => Err("Unrecongized suit")
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Suit::Heart => "H",
            Suit::Spade => "S",
            Suit::Diamond => "D",
            Suit::Club => "C"
        })
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, EnumIter)]
pub enum Rank {
    Ace = 0,
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
    King
}

impl FromStr for Rank {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rank::Ace),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "0"|"T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            _ => Err("Unknown rank")
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "0",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K"
        })
    }
}

fn mask(r : &Rank) -> usize {
    *r as usize
}

fn value(r : &Rank) -> i32 {
    match r {
        Rank::Ace => 1,
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ten => 10,
        Rank::Jack => 10,
        Rank::Queen => 10,
        Rank::King => 10
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl FromStr for Card {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 2 {
            return Err("Number of characters in `s` != 2");
        };
        let the_rank = Rank::from_str(&s[0..1])?;
        let the_suit = Suit::from_str(&s[1..2])?;
        Ok(Card { suit : the_suit, rank : the_rank })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(s: &str) -> Self {
        Self::from_str(&s).unwrap()
    }

    pub fn mask(&self) -> usize {
        mask(&self.rank)
    }

    pub fn value(&self) -> i32 {
        value(&self.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_to_string_1() {
        assert_eq!(Card { suit : Suit::Heart, rank : Rank::Three }.to_string(), "3H")
    }

    #[test]
    fn test_card_from_str_1() {
        assert_eq!(Card::from_str("AH").unwrap(), Card { suit : Suit::Heart, rank : Rank::Ace })
    }

    #[test]
    #[should_panic]
    fn test_card_from_str_2() {
        Card::from_str("AHX").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_from_str_3() {
        Card::from_str("A").unwrap();
    }

}
