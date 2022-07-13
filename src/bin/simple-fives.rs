// calculate points of all five-card hands possible in the deck

use cribbage::card::deck;
use cribbage::hand::Hand;
use itertools::Itertools;

fn main() {
    let deck = deck();
    for (icut, cut) in deck.iter().enumerate() {
        let mut hand_options = deck.clone();
        hand_options.remove(icut);
        for hand in hand_options.iter().cloned().combinations(4).map(|rs| Hand { hand : rs }) {
            println!("{} with cut {} scored {}", hand, cut, hand.score(cut));
        }
    }
}
