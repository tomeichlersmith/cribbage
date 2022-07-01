
enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
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
impl CardValue {
    fn fifteen_value(&self) -> u8 {
        match self {
            Ace => 1
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten|Jack|Queen|King => 10,
        }
    }
    fn run_value(&self) -> u8 {
        match self {
            Ace => 1
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13
        }
    }
}

// The four suits in a standard deck of cards
#[derive(PartialEq)]
enum Card {
    Spade(CardValue),
    Club(CardValue),
    Heart(CardValue),
    Diamond(CardValue)
}

// calculate the score of the input hand
//     a "hand" in this logic is five-cards along with the flip card
fn calculate_score(the_deck : &[Card; 52], hand : &[usize; 4], flip : usize) {
    let mut score : u8 = 0;

    // count flush points
    if hand.0.suit == hand.1.suit && hand.1.suit == hand.2.suit && hand.2.suit == hand.3.suit {
        score += 4;
        if hand.0.suit == flip.suit {
            score += 1;
        }
    }


    // count fifteen points
    let full_hand = [ hand.0, hand.1, hand.2, hand.3, flip ];


    // count runs

    score
}

fn main() {

    // the deck
    const THE_DECK : [Card; 52] = [
        Card::Spade(CardValue::Ace),
        Card { suit : Suit::Spade, value :  1 },
        Card { suit : Suit::Spade, value :  2 },
        Card { suit : Suit::Spade, value :  3 },
        Card { suit : Suit::Spade, value :  4 },
        Card { suit : Suit::Spade, value :  5 },
        Card { suit : Suit::Spade, value :  6 },
        Card { suit : Suit::Spade, value :  7 },
        Card { suit : Suit::Spade, value :  8 },
        Card { suit : Suit::Spade, value :  9 },
        Card { suit : Suit::Spade, value : 10 },
        Card { suit : Suit::Spade, value : 11 },
        Card { suit : Suit::Spade, value : 12 },
        Card { suit : Suit::Spade, value : 13 },
        Card { suit : Suit::Club , value :  1 },
        Card { suit : Suit::Club , value :  2 },
        Card { suit : Suit::Club , value :  3 },
        Card { suit : Suit::Club , value :  4 },
        Card { suit : Suit::Club , value :  5 },
        Card { suit : Suit::Club , value :  6 },
        Card { suit : Suit::Club , value :  7 },
        Card { suit : Suit::Club , value :  8 },
        Card { suit : Suit::Club , value :  9 },
        Card { suit : Suit::Club , value : 10 },
        Card { suit : Suit::Club , value : 11 },
        Card { suit : Suit::Club , value : 12 },
        Card { suit : Suit::Club , value : 13 },
        Card { suit : Suit::Heart, value :  1 },
        Card { suit : Suit::Heart, value :  2 },
        Card { suit : Suit::Heart, value :  3 },
        Card { suit : Suit::Heart, value :  4 },
        Card { suit : Suit::Heart, value :  5 },
        Card { suit : Suit::Heart, value :  6 },
        Card { suit : Suit::Heart, value :  7 },
        Card { suit : Suit::Heart, value :  8 },
        Card { suit : Suit::Heart, value :  9 },
        Card { suit : Suit::Heart, value : 10 },
        Card { suit : Suit::Heart, value : 11 },
        Card { suit : Suit::Heart, value : 12 },
        Card { suit : Suit::Heart, value : 13 },
        Card { suit : Suit::Diamond, value :  1 },
        Card { suit : Suit::Diamond, value :  2 },
        Card { suit : Suit::Diamond, value :  3 },
        Card { suit : Suit::Diamond, value :  4 },
        Card { suit : Suit::Diamond, value :  5 },
        Card { suit : Suit::Diamond, value :  6 },
        Card { suit : Suit::Diamond, value :  7 },
        Card { suit : Suit::Diamond, value :  8 },
        Card { suit : Suit::Diamond, value :  9 },
        Card { suit : Suit::Diamond, value : 10 },
        Card { suit : Suit::Diamond, value : 11 },
        Card { suit : Suit::Diamond, value : 12 },
        Card { suit : Suit::Diamond, value : 13 },
    ];

    println!("{} cards", THE_DECK.len());
    for c in THE_DECK {
        let suit_name = match c.suit {
            Suit::Spade => "Spade",
            Suit::Club  => "Club",
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond"
        };
        let val_name = match c.value {
            1 => String::from("Ace"),
            11 => String::from("Jack"),
            12 => String::from("Queen"),
            13 => String::from("King"),
            _ => c.value.to_string()
        };
        println!("{} of {}s", val_name, suit_name);
    }
}
