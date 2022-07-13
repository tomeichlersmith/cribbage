use crate::card::Card;
use crate::hand::Show;
use itertools::Itertools;

fn cards(s: &Show) -> Vec<Card> {
    let mut cs = s.hand.hand.to_vec();
    cs.push(s.cut);
    return cs
}

fn is_fifteen(cs: &[Card]) -> bool {
    let mut t = 0;
    for c in cs.iter() {
        t += c.value
    };
    return t == 15
}

pub trait Fifteens {
    fn cards(&self) -> Vec<Card>;

    fn score_fifteens(&self) -> i32 {
        let mut t: i32 = 0;
        for n in 2..5 {
            for xs in self.cards().iter().cloned().combinations(n) {
                if is_fifteen(&xs) {
                    t += 2
                }
            }
        }
        return t
    }
}

impl Fifteens for Show {
    fn cards(&self) -> Vec<Card> {
        cards(self)
    }
}

fn is_pair(c1: &Card, c2: &Card) -> bool {
    return c1.rank == c2.rank
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_score_fifteens_1() {
        let h = Show::new(&vec!["2♡", "3♡", "5♡", "T♡"], "5♣");
        assert_eq!(h.score_fifteens(), 8)
    }

    #[test]
    fn test_is_pair_1() {
        assert_eq!(
            is_pair(
                &Card::from_chars('K', '♡'),
                &Card::from_chars('K', '♠'),
            ),
            true,
        )
    }

    #[test]
    fn test_is_pair_2() {
        assert_eq!(
            is_pair(
                &Card::from_chars('A', '♢'),
                &Card::from_chars('A', '♣'),
            ),
            true,
        )
    }

    #[test]
    fn test_is_pair_3() {
        assert_eq!(
            is_pair(
                &Card::from_chars('A', '♢'),
                &Card::from_chars('A', '♣').ace_high(),
            ),
            true,
        )
    }

    #[test]
    fn test_is_pair_4() {
        assert_eq!(
            is_pair(
                &Card::from_chars('K', '♡'),
                &Card::from_chars('A', '♣'),
            ),
            false,
        )
    }

    #[test]
    fn test_is_pair_5() {
        assert_eq!(
            is_pair(
                &Card::from_chars('K', '♡'),
                &Card::from_chars('A', '♣').ace_high(),
            ),
            false,
        )
    }

    fn card_vec(cs: Vec<(char,char)>) -> Vec<Card> {
        cs.iter()
        .map(|(x,y)| Card::from_chars(*x, *y))
        .collect::<Vec<_>>()
    }

    fn test_fifteen(cs: Vec<(char,char)>, b: bool) {
        assert_eq!(
            is_fifteen(&card_vec(cs)),
            b,
        )
    }

    #[test]
    fn test_is_fifteen_1() {
        test_fifteen(vec![('7', '♡'), ('8', '♡')], true)
    }

    #[test]
    fn test_is_fifteen_2() {
        test_fifteen(vec![('5', '♡'), ('5', '♠'), ('5', '♣')], true)
    }

    #[test]
    fn test_is_fifteen_3() {
        test_fifteen(vec![('2', '♡'), ('3', '♠'), ('T', '♣')], true)
    }

    #[test]
    fn test_is_fifteen_4() {
        test_fifteen(vec![('A', '♡'), ('A', '♠'), ('3', '♠'), ('T', '♣')], true)
    }

    #[test]
    fn test_is_fifteen_5() {
        test_fifteen(vec![('A', '♡'), ('3', '♠'), ('4', '♠'), ('4', '♡'), ('3', '♢')], true)
    }

    #[test]
    fn test_is_fifteen_6() {
        test_fifteen(vec![('3', '♠'), ('4', '♡'), ('T', '♡')], false)
    }

    #[test]
    fn test_is_fifteen_7() {
        println!("{}", Card::from_chars('A', '♣').value);
        assert_eq!(
            is_fifteen(
                &vec![
                    Card::from_chars('A', '♡').ace_high(),
                    Card::from_chars('K', '♣'),
                    Card::from_chars('4', '♢'),
                ],
            ),
            true,
        )
    }

    #[test]
    fn test_show_fifteens_cards() {
        let s = Show::new(&vec!["2♡", "3♡", "4♡", "5♡"], "6♡");
        assert_eq!(
            s.cards(),
            vec!["2♡", "3♡", "4♡", "5♡", "6♡"]
                .iter()
                .map(|x| Card::from_str(x).unwrap())
                .collect::<Vec<_>>(),
        )
    }


}
