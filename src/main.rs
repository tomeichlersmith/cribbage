
use cribbage::card::Card;
use cribbage::hand::Show;
use cribbage::score::*;

fn main() {
    let h = Show::new(&vec!["2♡", "3♡", "5♡", "T♡"], "5♣");
    println!("{}" , h.score_fifteens())
}
