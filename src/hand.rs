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
/// holds in their hand (and thus a cut card must be given
/// in order to calculate a score).
#[derive(Eq, PartialEq, Debug)]
pub struct Hand {
    pub hand: Vec<Card>,
}

impl Hand {
    /// construct a new hand from a list of strings
    ///
    /// # Panics
    /// - if the hand is not of the correct size (4 cards)
    /// - if any of the strings provided cannot be deduced into a Card
    #[must_use]
    pub fn new(cs: &[&str]) -> Self {
        assert!(cs.len() == 4, "`Hand` must contain four `Card`s");
        Self {
            hand: cs
                .iter()
                .map(|x| Card::from_str(x).unwrap())
                .collect()
        }
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
    #[must_use]
    pub fn score(&self, cut : &Card) -> i32 {
        let mut s : i32 = 0;

        // flush
        if self.hand.iter().all(|&c| c.suit == self.hand[0].suit) {
            // there are no cards in the hand that have a different suit than the first card,
            //  ==> four card flush
            s += 4;
            if self.hand[0].suit == cut.suit {
                // 5 card flush
                s += 1;
            }
        }

        // player has a Jack matching suit of cut
        if self.hand.iter().any(|&c| c.suit == cut.suit && c.rank == Rank::Jack) {
            s += 1;
        }

        // construct full hand for cut-agnostic calculations
        let mut full_hand = self.hand.clone();
        full_hand.push(*cut);
        
        // count points worth fifteen
        for n in 2..5 {
            // loop through sub combinations of N cards
            for xs in full_hand.iter().copied().combinations(n) {
                let mut total = 0;
                for c in xs {
                    total += c.value();
                }
                if total == 15 {
                    s += 2;
                }
            }
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
            let ic : usize = c.mask();
            buckets[ic] += 1;
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
        write!(f, "[{} {} {} {}]", self.hand[0], self.hand[1], self.hand[2], self.hand[3])
    }
}

/// helper function for creating a hand from a slice of strings
#[must_use]
pub fn hand(cs: &[&str]) -> Hand {
    Hand::new(cs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn construct_and_print_hand() {
        let h = Hand::new(&["2H", "3H", "4H", "5H"]);
        println!("{}",h)
    }

    #[test]
    fn same_hand() {
        assert_eq!(
            Hand::new(&["2H", "3H", "4H", "5H"]),
            Hand::new(&["2H", "3H", "4H", "5H"]),
        );
    }

    #[test]
    fn test_hand_fn_1() {
        assert_eq!(
            hand(&["2H", "3H", "4H", "5H"]),
            Hand::new(&["2H", "3H", "4H", "5H"]),
        );
    }

    fn test_score(h : &[&str], c : &str) -> i32 {
        hand(h).score(&Card::from_str(c).unwrap())
    }

    #[test]
    fn test_score_flush_fifteens() {
        assert_eq!(test_score(&["2H","3H","5H","TH"],"5C"), 14)
    }

    #[test]
    fn test_score_nobs_fifteens() {
        assert_eq!(test_score(&["3H","5C","JH","QH"],"7H"), 7)
    }

    #[test]
    fn test_score_double_run() {
        assert_eq!(test_score(&["3H","4D","4C","5C"],"7H"), 12)
    }

    #[test]
    fn test_low_single_run() {
        assert_eq!(test_score(&["AH","2C","3D","4D"],"6H"), 6)
    }

    #[test]
    fn test_single_run_with_cut() {
        assert_eq!(test_score(&["AH","2C","3D","6H"],"4H"), 6)
    }

    #[test]
    fn test_high_run() {
        assert_eq!(test_score(&["0C","JD","QC","KH"],"3C"), 4)
    }

    #[test]
    fn test_triple_run() {
        assert_eq!(test_score(&["QC","KS","KD","JD"],"KC"), 15)
    }
}
