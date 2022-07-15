
use crate::card::{
    Rank,
    Suit,
    Card
};
use strum::IntoEnumIterator;
use itertools::Itertools;
use lazy_static::lazy_static;

// the full deck, static list of cards since there is only one full deck
lazy_static! {
    static ref THE_DECK : Vec<Card> = Rank::iter()
        .cartesian_product(Suit::iter())
        .map(|(r,s)| Card { rank : r, suit : s })
        .collect();
}

// return a reference to the full deck
pub fn full_deck() -> &'static Vec<Card> {
    &THE_DECK
}

// a view of the deck with the optionally input
//  cards excluded from it (as if they were already drawn)
pub fn part_deck(already_drawn : &[&Card]) -> Vec<&'static Card> {
    THE_DECK.iter().filter(|c| !already_drawn.contains(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_1() {
        assert_eq!(THE_DECK.len(), 52);
    }

    #[test]
    fn test_deck_2() {
        assert_eq!(deck(None), 52);
    }

    #[test]
    fn test_deck_3() {
        assert_eq!(deck(&[Card::from_str("5H")]).len(), 51)
    }
}
