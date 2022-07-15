// calculate points of all five-card hands possible in the deck

use cribbage::deck::{full_deck, part_deck};
use cribbage::hand::Hand;
use itertools::Itertools;

fn main() {
    for (icut, cut) in full_deck().iter().enumerate() {
        for hand in part_deck(&[cut]).iter().cloned().combinations(4).map(|rs| Hand { hand : rs }) {
            println!("{} with cut {} scored {}", hand, cut, hand.score(cut));
        }
    }
}
