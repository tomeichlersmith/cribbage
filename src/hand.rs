use crate::card::{ Rank, Card };
use std::fmt;
use std::str::FromStr;
use itertools::Itertools;

/// a scorable hand of cards
///
/// In cribbage, hands that can score points consist
/// of four cards held by a player and the "cut" card
/// revealed after hands are dealt and the crib cards are
/// chosen. This struct holds the four cards that a player
/// holds in their hand (called `hand`) and the card that
/// was flipped after the deal (called `cut`).
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Hand {
    pub hand: [Card;4],
    pub cut: Card,
}

impl Hand {
    /// construct a new hand from a list of strings
    ///
    /// # Panics
    /// - if any of the strings provided cannot be deduced into a Card
    #[must_use]
    pub fn new(cs: &[&str], c : &str) -> Self {
        Hand::from_cards(
            cs.iter().map(|x| Card::from_str(x).unwrap()).collect(),
            Card::from_str(c).unwrap()
            )
    }

    /// construct a new hand from a list of cards
    ///
    /// # Panics
    /// - if the number of cards provided for the hand is not 4
    #[must_use]
    pub fn from_cards(h : Vec<Card>, cut : Card) -> Self {
        assert!(h.len() == 4, "`Hand` must contain four `Card`s");
        let mut hand : [Card;4] = h.clone().try_into().unwrap();
        // we need to sort the hand here so that the derived
        // equality and hashing can work as expected
        // I don't expect this to be a large performance burden
        // since there are only four cards which are dictionary
        // sorted by their two identifying values
        hand.sort();
        Self { hand, cut }
    }

    /// this is where we score a hand given a specific cut card
    ///
    /// Points are scored in many different ways:
    /// - Four Card Flush: If the four cards in the hand are the same suit,
    ///     the player scores four points
    /// - Five Card Flush: If the player has a Four Card Flush and the cut 
    ///     is the same suit, then the player scores an additional point.
    /// - Nobs: The player scores a point if they have the Jack of the
    ///     same suit as the cut card
    /// - Fifteens: The player scores two points for all combinations of
    ///     cards whose values sum to 15 (face cards are all 10).
    /// - Runs: The player scores a point for each card participating in
    ///     a run of three or more cards (face cards maintain their rank).
    /// - Pairs: The player scores two points for all pairs of cards
    ///     with the same value.
    ///
    /// For Fifteens, Runs, and Pairs, the cut and the player's hand cards
    /// are all treated the same way.
    pub fn score(&self) -> usize {
        // need to calculate score
        let mut s : usize = 0;

        // flush
        if self.hand.iter().all(|&c| c.suit == self.hand[0].suit) {
            // there are no cards in the hand that have a different suit than the first card,
            //  ==> four card flush
            s += 4;
            if self.hand[0].suit == self.cut.suit {
                // 5 card flush
                s += 1;
            }
        }

        // player has a Jack matching suit of cut
        if self.hand.iter().any(|&c| c.suit == self.cut.suit && c.rank == Rank::Jack) {
            s += 1;
        }

        // construct full hand for cut-agnostic calculations
        let mut full_hand = self.hand.to_vec();
        full_hand.push(self.cut);
        
        // count points worth fifteen
        for n in 2..=5 {
            // each subset of cards that total fifteen
            //  is worth 2 points, so we go through all
            //  subsets of cards of length `n`,
            //  filter out the subsets that total 15,
            //  and then count them and multiply by 2
            s += full_hand
                .iter()
                .copied()
                .combinations(n)
                .filter(|subset| {
                    subset
                        .iter()
                        .map(|c| c.value())
                        .sum::<i32>() == 15
                })
                .count() * 2;
        }

        // run/pair scoring taken from a post on Code Golf:
        //  https://codegolf.stackexchange.com/a/5755

        // sort hand based off of card rank
        //   score tallying only happends when a empty bucket
        //   is enountered; this means the list of buckets 
        //   should be one longer than the number of possible
        //   ranks so that we can catch runs ending with the
        //   high card (Kings)
        let mut buckets = [0; 14];
        for c in full_hand {
            buckets[c.mask()] += 1;
        }

        let mut curr_run_len = 0;
        let mut curr_run_combos = 1;
        for b in buckets {
            if b > 0 {
                // calculate score of pairs
                s += b * (b-1);
                // track number in current run
                curr_run_len += 1;
                // track number of different combos
                // form this run
                curr_run_combos *= b;
            } else {
                if curr_run_len > 2 {
                    // run longer than 3
                    //  points for this run is length of run
                    //  times number of combos that can make it
                    s += curr_run_len * curr_run_combos;
                }
                // reset counters
                curr_run_len = 0;
                curr_run_combos = 1;
            }
        }
        s
    }
}

impl fmt::Display for Hand {
    /// print the string form of the hand
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {} {} {}] {}", self.hand[0], self.hand[1], self.hand[2], self.hand[3], self.cut)
    }
}

impl std::hash::Hash for Hand {
    /// hashing a hand is simply hashing all the cards
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.cut.hash(state);
        for c in self.hand {
            c.hash(state);
        }
        state.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn construct_and_print_hand() {
        let h = Hand::new(&["2H", "3H", "4H", "5H"],"6H");
        println!("{}",h)
    }

    #[test]
    fn same_hand() {
        assert_eq!(
            Hand::new(&["2H", "3H", "4H", "5H"],"6H"),
            Hand::new(&["2H", "3H", "4H", "5H"],"6H"),
        );
    }

    #[test]
    fn same_hand_diff_order() {
        assert_eq!(
            Hand::new(&["2H", "3H", "4H", "5H"],"6H"),
            Hand::new(&["3H", "2H", "4H", "5H"],"6H"),
            );
    }

    fn hash(hand : Hand) -> u64 {
        let mut hasher = DefaultHasher::new();
        hand.hash(&mut hasher);
        hasher.finish()
    }
    
    #[test]
    fn check_hash_same_order() {
        assert_eq!(
            hash(Hand::new(&["2H", "3H", "4H", "5H"],"6H")),
            hash(Hand::new(&["2H", "3H", "4H", "5H"],"6H")),
            )
    }

    #[test]
    fn check_hash_diff_order() {
        assert_eq!(
            hash(Hand::new(&["2H", "3H", "4H", "5H"],"6H")),
            hash(Hand::new(&["3H", "2H", "4H", "5H"],"6H")),
            )
    }

    #[test]
    #[should_panic]
    fn diff_hash_because_cut() {
        assert_eq!(
            hash(Hand::new(&["2H", "3H", "4H", "6H"],"5H")),
            hash(Hand::new(&["3H", "2H", "4H", "5H"],"6H")),
            )
    }

    fn test_score(h : &[&str], c : &str) -> usize {
        Hand::new(h,c).score()
    }

    #[test]
    fn five_flush_only() {
        assert_eq!(test_score(&["2H","4H","6H","8H"],"0H"), 5);
    }

    #[test]
    fn four_flush_only() {
        assert_eq!(test_score(&["2H","4H","6H","8H"],"0C"), 4);
    }

    #[test]
    fn score_flush_fifteens() {
        assert_eq!(test_score(&["2H","3H","5H","TH"],"5C"), 14)
    }

    #[test]
    fn score_nobs_fifteens() {
        assert_eq!(test_score(&["3H","5C","JH","QH"],"7H"), 7)
    }

    #[test]
    fn score_double_run() {
        assert_eq!(test_score(&["3H","4D","4C","5C"],"7H"), 12)
    }

    #[test]
    fn low_single_run() {
        assert_eq!(test_score(&["AH","2C","3D","4D"],"6H"), 6)
    }

    #[test]
    fn single_run_with_cut() {
        assert_eq!(test_score(&["AH","2C","3D","6H"],"4H"), 6)
    }

    #[test]
    fn high_run() {
        assert_eq!(test_score(&["0C","JD","QC","KH"],"3C"), 4)
    }

    #[test]
    fn triple_run() {
        assert_eq!(test_score(&["QC","KS","KD","JD"],"KC"), 15)
    }

    #[test]
    fn low_triple_run() {
        assert_eq!(test_score(&["AH","AD","AC","2C"],"3C"), 15)
    }
    
    #[test]
    fn double_double_run() {
        assert_eq!(test_score(&["AH","AD","3C","2C"],"3D"), 16)
    }

    #[test]
    fn double_double_run_with_nobs() {
        assert_eq!(test_score(&["JD","QC","KC","KD"],"QD"), 17);
    }

    #[test]
    fn triple_run_with_fifteen() {
        assert_eq!(test_score(&["3H","4D","4H","5C"],"4C"), 17);
    }

    #[test]
    fn fifteens_run_and_nobs() {
        assert_eq!(test_score(&["5H","5C","0C","JH"],"QH"),18);
    }

    #[test]
    fn fifteens_run_and_nonobs() {
        assert_eq!(test_score(&["5H","5C","0C","QH"],"JH"),17);
    }

    #[test]
    fn low_triple_run_2() {
        assert_eq!(test_score(&["AH","2C","3H","3C"],"3D"), 15);
    }

    #[test]
    fn fifteens_run_and_nonobs_2() {
        assert_eq!(test_score(&["3H","4C","4H","5C"],"4D"), 17);
    }

    #[test]
    fn five_card_fifteen() {
        assert_eq!(test_score(&["AC","2H","3H","4D"],"5C"),7);
    }
}
