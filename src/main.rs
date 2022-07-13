
use cribbage::hand::hand;
use cribbage::card::Card;
use std::str::FromStr;

fn main() {
    let h = hand(&vec!["2H", "3H", "5H", "TH"]);
    let s = h.score(&Card::from_str("5C").unwrap());
    println!("{} scored {}", h, s);
}
