
use std::collections::HashMap;
use crate::card::{
    Rank,
    Suit,
    Card
};
use crate::hand::Hand;
use strum::IntoEnumIterator;
use itertools::Itertools;

/// the full deck of cards
///
/// this does some iterator magics to construct all pairs of
/// suits and ranks and therefore all 52 cards
#[must_use]
pub fn full() -> Vec<Card> {
    Rank::iter()
        .cartesian_product(Suit::iter())
        .map(|(r,s)| Card { rank : r, suit : s })
        .collect()
}

/// a view of the deck with the input cards removed
///  (as if they were already drawn)
///
/// This is helpful for when getting all of the possible cut
/// cards given a set of cards that has already been drawn
#[must_use]
pub fn part(already_drawn : &[Card]) -> Vec<Card> {
    full().iter().filter(|c| !already_drawn.contains(c)).copied().collect()
}

/// the full set of all unique 5-card hands
///
/// this Look Up Table (LUT) is helpful for improving the speed
/// of more complicated simulations comparing different
/// strategies. It takes about 65s to construct the 12_994_800
/// different possible unique scorable hands.
#[must_use]
pub fn unique_scores() -> HashMap<Hand,usize> {
    let mut lut = HashMap::new();
    for cards in full().iter().copied().combinations(4) {
        for cut in part(&cards) {
            let hand = Hand::from_cards(&cards, cut);
            lut.insert(hand,hand.score());
        }
    }
    lut
}

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
        let one_drawn = part(&[card_drawn]);
        assert_eq!(one_drawn.len(), 51);
        assert!(!one_drawn.contains(&card_drawn));
    }
}
