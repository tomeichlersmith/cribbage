
// The four suits in a standard deck of cards
#[derive(PartialEq)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond
}

// A Card is uniquely defined by a suit and a value
//     For the purposes of runs,
//         A = 1, K = 13, Q = 12, J = 11
//     For the purposes of fifteens,
//         A = 1 OR 11, K = Q = J = 10
//
//     Since runs make more sense, we store the value
//     of the card as their run value AND NEED TO REMEMBER
//     TO ROUND >= 10 down to 10
struct Card {
    suit: Suit,
    value: u8
}

// calculate the score of the input hand
//     a "hand" in this logic is five-cards along with the flip card
fn calculate_score(hand : (Card,Card,Card,Card), flip : Card) -> u8 {
    let mut score : u8 = 0;
    
    // count flush points
    if hand.0.suit == hand.1.suit && hand.1.suit == hand.2.suit && hand.2.suit == hand.3.suit {
        score += 4;
        if hand.0.suit == flip.suit {
            score += 1;
        }
    }


    // count fifteen points

    // count runs

    score
}

fn main() {
    println!("Hello, world!");
}
