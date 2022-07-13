use std::{
    fmt,
    string,
    char,
    str,
};
use std::str::FromStr;
use itertools::Itertools;

// TODO fancy print suits using their UTF-8 symbols
pub const SUITS: [char; 4] = [
    'H',
    'S',
    'D',
    'C',
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
        // TODO wrap in Result object
        if !RANKS.contains(&rank) {
            panic!("Invalid `rank`");
        } else if !SUITS.contains(&suit) {
            panic!("Invalid `suit`");
        };

        // unwrap safe since we check for containment above
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
        assert_eq!(Card::from_chars('A', 'C'), Card::from_chars('A', 'C'))
    }

    #[test]
    fn test_card_equality_2() {
        assert_eq!(Card::from_chars('3', 'H'), Card::from_chars('3', 'H'))
    }

    #[test]
    fn test_card_equality_3() {
        assert_eq!(Card::from_chars('7', 'S'), Card::from_chars('7', 'S'))
    }

    #[test]
    fn test_card_equality_4() {
        assert_eq!(Card::from_chars('J', 'D'), Card::from_chars('J', 'D'))
    }

    #[test]
    fn test_card_inequality_1() {
        assert_ne!(Card::from_chars('A', 'C'), Card::from_chars('A', 'H'))
    }

    #[test]
    fn test_card_inequality_2() {
        assert_ne!(Card::from_chars('3', 'H'), Card::from_chars('A', 'S'))
    }

    #[test]
    fn test_card_inequality_3() {
        assert_ne!(Card::from_chars('7', 'S'), Card::from_chars('7', 'D'))
    }

    #[test]
    fn test_card_inequality_4() {
        assert_ne!(Card::from_chars('J', 'D'), Card::from_chars('J', 'C'))
    }

    #[test]
    fn test_card_rank() {
        assert_eq!(Card::from_chars('A', 'C').rank, 'A')
    }

    #[test]
    fn test_card_suit() {
        assert_eq!(Card::from_chars('A', 'C').suit, 'C')
    }

    #[test]
    fn test_card_mask_1() {
        assert_eq!(Card::from_chars('A', 'C').mask, 0)
    }

    #[test]
    fn test_card_mask_2() {
        assert_eq!(Card::from_chars('3', 'C').mask, 2)
    }

    #[test]
    fn test_card_mask_3() {
        assert_eq!(Card::from_chars('K', 'C').mask, 12)
    }

    #[test]
    fn test_card_ordering_1() {
        assert_eq!(Card::from_chars('A', 'C') < Card::from_chars('3', 'C'), true)
    }

    #[test]
    fn test_card_ordering_2() {
        assert_eq!(Card::from_chars('A', 'C') < Card::from_chars('3', 'H'), true)
    }

    #[test]
    fn test_card_ordering_3() {
        assert_eq!(Card::from_chars('A', 'H') < Card::from_chars('A', 'C'), false)
    }

    #[test]
    fn test_card_ordering_4() {
        assert_eq!(Card::from_chars('A', 'H') < Card::from_chars('A', 'D'), false)
    }

    #[test]
    fn test_card_ordering_5() {
        assert_eq!(Card::from_chars('A', 'D') < Card::from_chars('A', 'C'), false)
    }

    #[test]
    fn test_card_ordering_6() {
        assert_eq!(Card::from_chars('A', 'S') < Card::from_chars('A', 'D'), false)
    }

    #[test]
    fn test_card_to_string_1() {
        assert_eq!(Card::from_chars('A', 'S').to_string(), "AS")
    }

    #[test]
    fn test_card_to_string_2() {
        assert_eq!(Card::from_chars('K', 'H').to_string(), "KH")
    }

    #[test]
    fn test_card_from_str_1() {
        assert_eq!(Card::from_str("AH").unwrap(), Card::from_chars('A', 'H'))
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

    #[test]
    fn test_deck_1() {
        assert_eq!(deck().len(), 52);
    }
}
