
// A Card is uniquely defined by a suit and a value
//
//     For the purposes of runs, (aka "rank")
//         A = 1, K = 13, Q = 12, J = 11
//     For the purposes of fifteens, (aka "value")
//         A = 1 OR 11, K = Q = J = 10
//
//     Since runs make more sense, we store the value
//     of the card as their run value AND NEED TO REMEMBER
//     TO ROUND >= 10 down to 10
#[derive(PartialEq,Debug)]
enum Rank {
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

impl Rank {
    fn value(&self) -> u8 {
       match self {
           Rank::Ace => 1,
           Rank::Two => 2,
           Rank::Three => 3,
           Rank::Four => 4,
           Rank::Five => 5,
           Rank::Six => 6,
           Rank::Seven => 7,
           Rank::Eight => 8,
           Rank::Nine => 9,
           Rank::Ten|Rank::Jack|Rank::Queen|Rank::King => 10
       }
    }
    fn rank(&self) -> u8 {
       match self {
           Rank::Ace => 1,
           Rank::Two => 2,
           Rank::Three => 3,
           Rank::Four => 4,
           Rank::Five => 5,
           Rank::Six => 6,
           Rank::Seven => 7,
           Rank::Eight => 8,
           Rank::Nine => 9,
           Rank::Ten => 10,
           Rank::Jack => 11,
           Rank::Queen => 12,
           Rank::King => 13,
       }
    }
}

// The four suits in a standard deck of cards
#[derive(PartialEq,Debug)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond
}

struct Card {
    suit : Suit,
    rank : Rank
}

impl Card {
    fn value(&self) -> u8 {
        self.rank.value()
    }
    fn rank(&self) -> u8 {
        self.rank.rank()
    }
}

// we need to break a hand into all of its subsets
fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                             .map(|(_, element)| element)
                             .collect()
     }).collect()
}

// calculate the score of the input hand
//     a "hand" in this logic is five-cards along with the flip card
fn calculate(hand : (&Card, &Card, &Card, &Card), flip : &Card) -> u8 {
    let mut score : u8 = 0;

    // count flush points
    if hand.0.suit == hand.1.suit && hand.1.suit == hand.2.suit && hand.2.suit == hand.3.suit {
        score += 4;
        if hand.0.suit == flip.suit {
            score += 1;
        }
    }

    // check for special Jack
    for c in hand.iter() {
        if c.rank() == 11 && c.suit == flip.suit {
            score += 1
        }
    }

    // now we don't care about the flip/hand difference
    let full_hand = vec![ hand.0, hand.1, hand.2, hand.3, flip ];
    
    // count fifteens, runs, and pairs
    let all_combos = powerset(&full_hand);
    for set in all_combos {
        // score fifteens
        let mut rank_total = 0;
        for c in set.iter() {
            rank_total += c.value()
        }
        if rank_total == 15 {
            score += 2
        }

        // score pairs
        if set.len() == 2 && set[0].rank() == set[1].rank() {
            score += 2
        } else if set.len() >= 3 {

        }
    }

    // sort the hand by run value to check for runs while looping
    //full_hand.sort_by(|lhs, rhs|, lhs.value(InterpretType::Run) < rhs.value(InterpretType::Run));
//    let mut in_a_row = 0;
//    let mut last_run_val = 100; // dummy value
//    for c in full_hand {
//        let run_val = c.value(InterpretType::Run);
//        match run_val {
//            last_run_val|{last_run_val+1} => {
//                last_run_val = run_val;
//                in_a_row += 1;
//            }
//            _ => {
//                last_run_val = run_val;
//                in_a_row = 0;
//            }
//        }
//    }
//
//    if in_a_row >= 3 {
//        score += in_a_row
//    }

    score
}

fn main() {
    // the deck
    const THE_DECK : [Card; 52] = [
        Card { suit : Suit::Spade, rank : Rank::Ace },
        Card { suit : Suit::Spade, rank : Rank::Two},
        Card { suit : Suit::Spade, rank : Rank::Three},
        Card { suit : Suit::Spade, rank : Rank::Four},
        Card { suit : Suit::Spade, rank : Rank::Five},
        Card { suit : Suit::Spade, rank : Rank::Six},
        Card { suit : Suit::Spade, rank : Rank::Seven},
        Card { suit : Suit::Spade, rank : Rank::Eight},
        Card { suit : Suit::Spade, rank : Rank::Nine},
        Card { suit : Suit::Spade, rank : Rank::Ten},
        Card { suit : Suit::Spade, rank : Rank::Jack},
        Card { suit : Suit::Spade, rank : Rank::Queen},
        Card { suit : Suit::Spade, rank : Rank::King},
        Card { suit : Suit::Club, rank : Rank::Ace},
        Card { suit : Suit::Club, rank : Rank::Two},
        Card { suit : Suit::Club, rank : Rank::Three},
        Card { suit : Suit::Club, rank : Rank::Four},
        Card { suit : Suit::Club, rank : Rank::Five},
        Card { suit : Suit::Club, rank : Rank::Six},
        Card { suit : Suit::Club, rank : Rank::Seven},
        Card { suit : Suit::Club, rank : Rank::Eight},
        Card { suit : Suit::Club, rank : Rank::Nine},
        Card { suit : Suit::Club, rank : Rank::Ten},
        Card { suit : Suit::Club, rank : Rank::Jack},
        Card { suit : Suit::Club, rank : Rank::Queen},
        Card { suit : Suit::Club, rank : Rank::King},
        Card { suit : Suit::Heart, rank : Rank::Ace},
        Card { suit : Suit::Heart, rank : Rank::Two},
        Card { suit : Suit::Heart, rank : Rank::Three},
        Card { suit : Suit::Heart, rank : Rank::Four},
        Card { suit : Suit::Heart, rank : Rank::Five},
        Card { suit : Suit::Heart, rank : Rank::Six},
        Card { suit : Suit::Heart, rank : Rank::Seven},
        Card { suit : Suit::Heart, rank : Rank::Eight},
        Card { suit : Suit::Heart, rank : Rank::Nine},
        Card { suit : Suit::Heart, rank : Rank::Ten},
        Card { suit : Suit::Heart, rank : Rank::Jack},
        Card { suit : Suit::Heart, rank : Rank::Queen},
        Card { suit : Suit::Heart, rank : Rank::King},
        Card { suit : Suit::Diamond, rank : Rank::Ace},
        Card { suit : Suit::Diamond, rank : Rank::Two},
        Card { suit : Suit::Diamond, rank : Rank::Three},
        Card { suit : Suit::Diamond, rank : Rank::Four},
        Card { suit : Suit::Diamond, rank : Rank::Five},
        Card { suit : Suit::Diamond, rank : Rank::Six},
        Card { suit : Suit::Diamond, rank : Rank::Seven},
        Card { suit : Suit::Diamond, rank : Rank::Eight},
        Card { suit : Suit::Diamond, rank : Rank::Nine},
        Card { suit : Suit::Diamond, rank : Rank::Ten},
        Card { suit : Suit::Diamond, rank : Rank::Jack},
        Card { suit : Suit::Diamond, rank : Rank::Queen},
        Card { suit : Suit::Diamond, rank : Rank::King},
    ];

    println!("{}" , calculate(
            (&THE_DECK[0],&THE_DECK[1],&THE_DECK[2],&THE_DECK[3]),&THE_DECK[4]));
}
