
use cribbage::hand::hand;
use cribbage::card::Card;
use std::str::FromStr;

fn main() {
    let h = hand(&["2H", "3H", "5H", "TH"]);
    let c = Card::from_str("5C").unwrap();
    let s = h.score(&c);
    println!("{} with cut {} scored {}", h, c, s);
}

