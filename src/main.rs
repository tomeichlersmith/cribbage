
use cribbage::hand::Hand;

fn main() {
    let h = Hand::new(&["2H", "3H", "5H", "TH"],"5C");
    let s = h.score();
    println!("{} scored {}", h, s);
}

