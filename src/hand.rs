use crate::card::{
    Card,
    deck,
};
use rand::{
    thread_rng,
    seq::SliceRandom, // `choose_multiple`
};

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

    pub fn rand() -> Self {
        let mut rng = thread_rng();
        Hand {
            hand: deck()
                .choose_multiple(&mut rng, 4)
                .cloned()
                .collect()
        }
    }
}

pub fn hand(cs: &[&str]) -> Hand {
    Hand::new(cs)
}

#[derive(Eq, PartialEq, Debug)]
pub struct Show {
    pub hand: Hand,
    pub cut: Card,
}

impl Show {
    pub fn new(hand: &[&str], cut: &str) -> Self {
        Show {
            hand: Hand::new(hand),
            cut: Card::new(cut),
        }
    }

    pub fn rand() -> Self {
        let mut rng = thread_rng();
        let cs: Vec<Card> = deck()
            .choose_multiple(&mut rng, 5)
            .cloned()
            .collect();
        Show {
            hand: Hand {
                hand: cs[0..3].to_vec()
            },
            cut: cs[4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_constructor_1() {
        Hand::new(&["2♡", "3♡", "4♡", "5♡"]);
    }

    #[test]
    fn test_hand_equality_1() {
        assert_eq!(
            Hand::new(&vec!["2♡", "3♡", "4♡", "5♡"]),
            Hand::new(&["2♡", "3♡", "4♡", "5♡"]),
        );
    }

    #[test]
    fn test_hand_rand_1() {
        assert_eq!(
            Hand::rand().hand.len(),
            4,
        );
    }

    #[test]
    fn test_hand_fn_1() {
        assert_eq!(
            hand(&["2♡", "3♡", "4♡", "5♡"]),
            Hand::new(&vec!["2♡", "3♡", "4♡", "5♡"]),
        );
    }

    #[test]
    fn test_show_constructor_1() {
        Show::new(&vec!["2♡", "3♡", "4♡", "5♡"], "6♡");
    }

    #[test]
    fn test_show_constructor_2() {
        let s = Show::new(&["2♡", "3♡", "4♡", "5♡"], "6♡");
        assert_eq!(
            s.hand,
            Hand::new(&["2♡", "3♡", "4♡", "5♡"]),
        )
    }

    #[test]
    fn test_show_constructor_3() {
        let s = Show::new(&vec!["2♡", "3♡", "4♡", "5♡"], "6♡");
        assert_eq!(
            s.cut,
            Card::new("6♡"),
        )
    }
}
