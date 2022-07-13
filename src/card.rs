use std::{
    fmt,
    string,
    char,
    str,
};
use std::str::FromStr;
use itertools::Itertools;

// TODO allow ['H', 'S', 'D', 'C']
pub const SUITS: [char; 4] = [
    '♡',
    '♠',
    '♢',
    '♣',
];

pub const RANKS: [char; 13] = [
// rank,    value,  mask,
    'A', //     1,     0,
    '2', //     2,     1,
    '3', //     3,     2,
    '4', //     4,     3,
    '5', //     5,     4,
    '6', //     6,     5,
    '7', //     7,     6,
    '8', //     8,     7,
    '9', //     9,     8,
    'T', //    10,     9,
    'J', //    10,    10,
    'Q', //    10,    11,
    'K', //    10,    12,
// Ace high:
//  'A',        1,    13,
];

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Card {
    pub mask: i32,
    pub suit: char,
    pub rank: char,
    pub value: i32,
}

impl Card {
    pub fn from_chars(rank: char, suit: char) -> Self {
        if !RANKS.contains(&rank) {
            panic!("Invalid `rank`");
        } else if !SUITS.contains(&suit) {
            panic!("Invalid `suit`");
        };
        let mask = RANKS.iter().position(|x| x == &rank).unwrap() as i32;
        let value = mask + 1;
        let value = match value > 10 {
            true => 10,
            _ => value
        };
        Self {
            mask,
            suit,
            rank,
            value,
        }
    }

    pub fn new(s: &str) -> Self {
        Self::from_str(&s).unwrap()
    }

    pub fn ace_high(&self) -> Self {
        if self.rank != 'A' {
            panic!("`rank` != 'A'")
        }
        Self {
            mask: 13,
            suit: self.suit,
            rank: self.rank,
            value: self.value,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl FromStr for Card {
    type Err = string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 2 {
            panic!("Number of characters in `s` != 2")
        };
        Ok(Card::from_chars(
            s.chars().nth(0).unwrap(),
            s.chars().nth(1).unwrap(),
        ))
    }
}

pub fn deck() -> Vec<Card> {
    RANKS
        .iter()
        .cartesian_product(SUITS.iter())
        .map(|rs| Card::from_chars(*rs.0, *rs.1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_equality_1() {
        assert_eq!(Card::from_chars('A', '♣'), Card::from_chars('A', '♣'))
    }

    #[test]
    fn test_card_equality_2() {
        assert_eq!(Card::from_chars('3', '♡'), Card::from_chars('3', '♡'))
    }

    #[test]
    fn test_card_equality_3() {
        assert_eq!(Card::from_chars('7', '♠'), Card::from_chars('7', '♠'))
    }

    #[test]
    fn test_card_equality_4() {
        assert_eq!(Card::from_chars('J', '♢'), Card::from_chars('J', '♢'))
    }

    #[test]
    fn test_card_inequality_1() {
        assert_ne!(Card::from_chars('A', '♣'), Card::from_chars('A', '♡'))
    }

    #[test]
    fn test_card_inequality_2() {
        assert_ne!(Card::from_chars('3', '♡'), Card::from_chars('A', '♠'))
    }

    #[test]
    fn test_card_inequality_3() {
        assert_ne!(Card::from_chars('7', '♠'), Card::from_chars('7', '♢'))
    }

    #[test]
    fn test_card_inequality_4() {
        assert_ne!(Card::from_chars('J', '♢'), Card::from_chars('J', '♣'))
    }

    #[test]
    fn test_card_rank() {
        assert_eq!(Card::from_chars('A', '♣').rank, 'A')
    }

    #[test]
    fn test_card_suit() {
        assert_eq!(Card::from_chars('A', '♣').suit, '♣')
    }

    #[test]
    fn test_card_mask_1() {
        assert_eq!(Card::from_chars('A', '♣').mask, 0)
    }

    #[test]
    fn test_card_mask_2() {
        assert_eq!(Card::from_chars('3', '♣').mask, 2)
    }

    #[test]
    fn test_card_mask_3() {
        assert_eq!(Card::from_chars('K', '♣').mask, 12)
    }

    #[test]
    fn test_card_mask_ace_high() {
        assert_eq!(Card::from_chars('A', '♣').ace_high().mask, 13)
    }

    #[test]
    fn test_card_ordering_1() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('3', '♣'), true)
    }

    #[test]
    fn test_card_ordering_2() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('3', '♡'), true)
    }

    #[test]
    fn test_card_ordering_3() {
        assert_eq!(Card::from_chars('A', '♡') < Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_ordering_4() {
        assert_eq!(Card::from_chars('A', '♡') < Card::from_chars('A', '♢'), true)
    }

    #[test]
    fn test_card_ordering_5() {
        assert_eq!(Card::from_chars('A', '♢') < Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_ordering_6() {
        assert_eq!(Card::from_chars('A', '♠') < Card::from_chars('A', '♢'), true)
    }

    #[test]
    fn test_card_ordering_ace_high_1() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('A', '♣').ace_high(), true)
    }

    #[test]
    fn test_card_ordering_ace_high_2() {
        assert_eq!(Card::from_chars('A', '♣') < Card::from_chars('A', '♠').ace_high(), true)
    }

    #[test]
    fn test_card_ordering_ace_high_3() {
        assert_eq!(Card::from_chars('A', '♣').ace_high() > Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_ordering_ace_high_4() {
        assert_eq!(Card::from_chars('A', '♠').ace_high() > Card::from_chars('A', '♣'), true)
    }

    #[test]
    fn test_card_to_string_1() {
        assert_eq!(Card::from_chars('A', '♠').to_string(), "A♠")
    }

    #[test]
    fn test_card_to_string_2() {
        assert_eq!(Card::from_chars('K', '♡').to_string(), "K♡")
    }

    #[test]
    fn test_card_ace_high_1() {
        Card::from_chars('A', '♡').ace_high();
    }

    #[test]
    #[should_panic]
    fn test_card_ace_high_2() {
        Card::from_chars('K', '♡').ace_high();
    }

    #[test]
    fn test_card_from_str_1() {
        assert_eq!(Card::from_str("A♡").unwrap(), Card::from_chars('A', '♡'))
    }

    #[test]
    #[should_panic]
    fn test_card_from_str_2() {
        Card::from_str("A♡X").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_card_from_str_3() {
        Card::from_str("A").unwrap();
    }

    #[test]
    fn test_deck_1() {
        assert_eq!(deck().len(), 52);
    }
}
