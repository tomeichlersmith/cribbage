
use crate::card::{
    Rank,
    Suit,
    Card
};
//use crate::hand::Hand;
use strum::IntoEnumIterator;
use itertools::Itertools;

// the full deck
pub fn full_deck() -> Vec<Card> {
    Rank::iter()
        .cartesian_product(Suit::iter())
        .map(|(r,s)| Card { rank : r, suit : s })
        .collect()
}

// a view of the deck with the input cards removed
//  (as if they were already drawn)
pub fn part_deck(already_drawn : &[&Card]) -> Vec<Card> {
    full_deck().iter().filter(|c| !already_drawn.contains(c)).cloned().collect()
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
    fn test_deck_1() {
        assert_eq!(full_deck().len(), 52);
    }

    #[test]
    fn test_deck_3() {
        assert_eq!(part_deck(&[&Card::from_str("5H").unwrap()]).len(), 51)
    }
}
