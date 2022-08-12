use crate::card::Card;
use crate::hand::Hand;
use rand::prelude::*;
use itertools::Itertools;

/// the Strategy trait which implements how four cards
/// out of the input list of cards is chosen to be kept
trait Strategy {
    fn choose(&mut self, cards_dealt : &Vec<Card>) -> [Card; 4];
}

/// the RandStrat simply randomly chooses four of the cards dealt to
/// it to be kept
///
/// it holds its own RNG so that it can be compared to other strategies
/// without interfering with the central RNG that will do the dealing
/// of cards
struct RandStrat<T : Rng> {
    rng : T
}

impl<T : Rng> Strategy for RandStrat<T> {
    fn choose (&mut self, cards_dealt : &Vec<Card>) -> [Card; 4] {
        cards_dealt
            .choose_multiple(&mut self.rng, 4)
            .copied()
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap()
    }
}

/// attempt to get maximum current score of four cards
struct MaxCurrentScore {}

impl Strategy for MaxCurrentScore {
    fn choose(&mut self, cards_dealt : &Vec<Card>) -> [Card; 4] {
        cards_dealt
            .into_iter()
            .combinations(4)
            .max_by_key(|cards| Hand::score_nocut(cards))
            .unwrap()
            .into_iter()
            .map(|c| c.clone())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap()
    }
}
