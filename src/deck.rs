
use crate::card::{
    Rank,
    Suit,
    Card
};
//use crate::hand::Hand;
use strum::IntoEnumIterator;
use itertools::Itertools;

// the full deck
#[must_use]
pub fn full() -> Vec<Card> {
    Rank::iter()
        .cartesian_product(Suit::iter())
        .map(|(r,s)| Card { rank : r, suit : s })
        .collect()
}

// a view of the deck with the input cards removed
//  (as if they were already drawn)
#[must_use]
pub fn part(already_drawn : &[Card]) -> Vec<Card> {
    full().iter().filter(|c| !already_drawn.contains(c)).copied().collect()
}

// deal cards in groups of N with the input cards removed
//pub fn deal(already_drawn : &[&Card], hand_size : usize) -> Vec<Card> {
//    full_deck()
//        .iter()
//        .filter(|c| !already_drawn.contains(c))
//        .combinations(hand_size)
//        .cloned()
//        .map(|rs| Hand {hand: rs})
//        .collect()
//}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn deck_corect_len() {
        assert_eq!(full().len(), 52);
    }

    #[test]
    fn one_drawn_from_deck() {
        let card_drawn = Card::from_str("5H").unwrap();
        let one_drawn = part(&[card_drawn.clone()]);
        assert_eq!(one_drawn.len(), 51);
        assert!(!one_drawn.contains(&card_drawn));
    }
}
