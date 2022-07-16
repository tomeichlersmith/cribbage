use crate::card::{ Rank, Card };
use std::fmt;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug)]
pub struct Hand {
    pub hand: Vec<Card>,
}

impl Hand {
    pub fn new(cs: &[&str]) -> Self {
        if cs.len() != 4 {
            panic!("`Hand` must contain four `Card`s")
        }
        Hand {
            hand: cs
                .iter()
                .map(|x| Card::new(x))
                .collect()
        }
    }

    // this is where we score a hand given a specific cut card
    //
    // TODO
    //  - optional cut? for potential optimization where player is trying
    //    to decide which four cards to use
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
            for xs in full_hand.iter().cloned().combinations(n) {
                let mut total = 0;
                for c in xs {
                    total += c.value()
                }
                if total == 15 {
                    s += 2
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
            buckets[ic] += 1
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
                    s += curr_run_len * curr_run_combos
                }
                // reset counters
                curr_run_len = 0;
                curr_run_combos = 1;
            }
        }
        return s
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {} {} {}]", self.hand[0], self.hand[1], self.hand[2], self.hand[3])
    }
}

pub fn hand(cs: &[&str]) -> Hand {
    Hand::new(cs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_hand_constructor_1() {
        Hand::new(&["2H", "3H", "4H", "5H"]);
    }

    #[test]
    fn test_hand_equality_1() {
        assert_eq!(
            Hand::new(&vec!["2H", "3H", "4H", "5H"]),
            Hand::new(&["2H", "3H", "4H", "5H"]),
        );
    }

    #[test]
    fn test_hand_fn_1() {
        assert_eq!(
            hand(&["2H", "3H", "4H", "5H"]),
            Hand::new(&vec!["2H", "3H", "4H", "5H"]),
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
}
