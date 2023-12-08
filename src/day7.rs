use core::panic;
use std::{cmp::Ordering, collections::HashMap, iter::zip};

use nom::{
    Parser,
    IResult,
    branch::alt,
    character::complete::one_of,
    combinator::{all_consuming, value},
    bytes::complete::{tag,take_while1},
    multi::{count,separated_list1}, sequence::separated_pair,
};

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum HandType {
    HighCard,//lowest
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,//highest
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
enum CamelCard {
    Num(u8),//smallest
    T,
    J,
    Q,
    K,
    A,//largest
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd)]
#[allow(dead_code)]
pub struct CamelCardsHand {
    cards: Vec<CamelCard>,
    bid_value: u64,
}

impl Ord for CamelCardsHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();
        if self_type > other_type {
            return Ordering::Greater;
        }
        if self_type < other_type {
            return Ordering::Less;
        }
        let my_iter = zip(self.cards.iter(), other.cards.iter());
        for (my_card, other_card) in my_iter {
            if my_card > other_card {
                return Ordering::Greater;
            }
            if my_card < other_card {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl CamelCardsHand {
    fn hand_type(&self) -> HandType {
        let card_counter = self.count_card_types();
        match card_counter.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {//Three of a kind OR Two pair
                match card_counter.iter().map(|(_,x)| x).max().unwrap() {
                    3 => HandType::ThreeOfAKind,
                    2 => HandType::TwoPair,
                    _ => unreachable!("Unexpected match"),
                }
            },
            2 => {//Four of a kind OR Full House
                match card_counter.iter().map(|(_,x)| x).min().unwrap() {
                    2 => HandType::FullHouse,
                    1 => HandType::FourOfAKind,
                    _ => unreachable!("Unexpected match"),
                }
            },
            1 => HandType::FiveOfAKind,
            _ => unreachable!("Error while counting camel cards"),
        }
    }

    fn count_card_types(&self) -> HashMap<CamelCard,u8> {
        let mut card_counter = HashMap::<CamelCard,u8>::new();
        for ccard in self.cards.iter() {
            if card_counter.contains_key(&ccard) {
                let foo = card_counter.get_mut(&ccard).unwrap();
                *foo += 1;
            } else {
                card_counter.insert(*ccard, 1);
            }
        }
        card_counter
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<CamelCardsHand> {
    match all_consuming(separated_list1(tag("\n"),parse_line))
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_line(input: &str) -> IResult<&str,CamelCardsHand> {
    separated_pair(parse_camel_cards, tag(" "), parse_num)
        .map(|(vec,bid)| CamelCardsHand {cards: vec, bid_value: bid})
        .parse(input)
}

fn parse_camel_cards(input: &str) -> IResult<&str, Vec<CamelCard>> {
    count(
        alt((
                value(CamelCard::A, tag("A")),
                value(CamelCard::K, tag("K")),
                value(CamelCard::Q, tag("Q")),
                value(CamelCard::J, tag("J")),
                value(CamelCard::T, tag("T")),
                one_of("23456789").map(|x: char| CamelCard::Num(x.to_string().parse::<u8>().unwrap())),
                )),
        5)
        .parse(input)
}

fn parse_num(input: &str) -> IResult<&str, u64> {
    take_while1(char::is_numeric)
        .map(|x: &str| x.parse::<u64>().unwrap())
        .parse(input)
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[CamelCardsHand]) -> u64 {
    let mut hands = input.to_vec();
    hands.sort_unstable_by(|a,b| a.cmp(&b));
    hands.into_iter()
        .enumerate()
        .map(|(i,score)| {
            (i as u64 + 1) * score.bid_value
        })
        .sum()
}

//#[aoc(day7, part2)]
//pub fn solve_part2(input: &Almanac) -> i64 {
//}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

#[test]
    fn test_camel_card() {
        assert_eq!(CamelCard::A > CamelCard::K, true);
        assert_eq!(CamelCard::Num(8) > CamelCard::Num(2), true);
    }
    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_day7_parser() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input[0],
            CamelCardsHand {cards: vec![CamelCard::Num(3), CamelCard::Num(2),
            CamelCard::T, CamelCard::Num(3), CamelCard::K], bid_value: 765}
            );
        assert_eq!(input.len(), 5);
        assert_eq!(input[4],
            CamelCardsHand {cards: vec![CamelCard::Q, CamelCard::Q,
            CamelCard::Q, CamelCard::J, CamelCard::A], bid_value: 483}
            );
    }
    
    #[test]
    fn test_camel_cards_ordering_p1() {
        let a = CamelCardsHand {
            cards: vec![CamelCard::A,CamelCard::A,CamelCard::A,CamelCard::A,CamelCard::A],
            bid_value: 69};
        let b = CamelCardsHand {
            cards: vec![CamelCard::A,CamelCard::A,CamelCard::A,CamelCard::A,CamelCard::A],
            bid_value: 70};
        assert_eq!(a.cmp(&b),Ordering::Equal);
        let c = CamelCardsHand {
            cards: vec![CamelCard::K,CamelCard::A,CamelCard::A,CamelCard::A,CamelCard::A],
            bid_value: 69};
        let d = CamelCardsHand {
            cards: vec![CamelCard::Q,CamelCard::A,CamelCard::A,CamelCard::A,CamelCard::A],
            bid_value: 69};
        assert_eq!(c.cmp(&d),Ordering::Greater);
        let e = CamelCardsHand {
            cards: vec![CamelCard::A,CamelCard::A,CamelCard::Num(2),CamelCard::A,CamelCard::A],
            bid_value: 69};
        let f = CamelCardsHand {
            cards: vec![CamelCard::A,CamelCard::A,CamelCard::Num(7),CamelCard::A,CamelCard::A],
            bid_value: 70};
        assert_eq!(e.cmp(&f),Ordering::Less);
    }

    #[test]
    fn test_camel_cards_ordering_p2() {
            let a = CamelCardsHand {
                cards: vec![CamelCard::Num(3),CamelCard::Num(2),CamelCard::T,CamelCard::Num(3),
                CamelCard::K], bid_value: 765};
            let b = CamelCardsHand {
                cards: vec![CamelCard::K,CamelCard::T,CamelCard::J,CamelCard::J,
                CamelCard::T], bid_value: 220};
            let c = CamelCardsHand {
                cards: vec![CamelCard::K,CamelCard::K,CamelCard::Num(6),CamelCard::Num(7),
                CamelCard::Num(7)], bid_value: 28};
            let d = CamelCardsHand {
                cards: vec![CamelCard::T,CamelCard::Num(5),CamelCard::Num(5),CamelCard::J,
                CamelCard::Num(5)], bid_value: 684};
            let e = CamelCardsHand {
                cards: vec![CamelCard::Q,CamelCard::Q,CamelCard::Q,CamelCard::J,
                CamelCard::A], bid_value: 483};
            assert_eq!(a.cmp(&b), Ordering::Less);
            assert_eq!(a.cmp(&c), Ordering::Less);
            assert_eq!(a.cmp(&d), Ordering::Less);
            assert_eq!(a.cmp(&e), Ordering::Less);
            assert_eq!(b.cmp(&c), Ordering::Less);
            assert_eq!(b.cmp(&d), Ordering::Less);
            assert_eq!(b.cmp(&e), Ordering::Less);
            assert_eq!(c.cmp(&d), Ordering::Less);
            assert_eq!(c.cmp(&e), Ordering::Less);
            assert_eq!(d.cmp(&e), Ordering::Less);
            // from the other side
            assert_eq!(e.cmp(&a), Ordering::Greater);
            assert_eq!(e.cmp(&b), Ordering::Greater);
            assert_eq!(e.cmp(&c), Ordering::Greater);
            assert_eq!(e.cmp(&d), Ordering::Greater);
            assert_eq!(d.cmp(&a), Ordering::Greater);
            assert_eq!(d.cmp(&b), Ordering::Greater);
            assert_eq!(d.cmp(&c), Ordering::Greater);
            assert_eq!(c.cmp(&a), Ordering::Greater);
            assert_eq!(c.cmp(&b), Ordering::Greater);
            assert_eq!(b.cmp(&a), Ordering::Greater);

            assert_ne!(a.cmp(&e), Ordering::Equal);
    }

    #[test]
    fn test_camel_cards_sorting() {
        let mut input = vec![
            CamelCardsHand {
                cards: vec![CamelCard::Num(3),CamelCard::Num(2),CamelCard::T,CamelCard::Num(3),
                CamelCard::K], bid_value: 765},
            CamelCardsHand {
                cards: vec![CamelCard::T,CamelCard::Num(5),CamelCard::Num(5),CamelCard::J,
                CamelCard::Num(5)], bid_value: 684},
            CamelCardsHand {
                cards: vec![CamelCard::K,CamelCard::K,CamelCard::Num(6),CamelCard::Num(7),
                CamelCard::Num(7)], bid_value: 28},
            CamelCardsHand {
                cards: vec![CamelCard::K,CamelCard::T,CamelCard::J,CamelCard::J,
                CamelCard::T], bid_value: 220},
            CamelCardsHand {
                cards: vec![CamelCard::Q,CamelCard::Q,CamelCard::Q,CamelCard::J,
                CamelCard::A], bid_value: 483},
        ];
        input.sort_unstable_by(|a,b| a.cmp(&b));
        assert_eq!(input,
            vec![
            CamelCardsHand {
                cards: vec![CamelCard::Num(3),CamelCard::Num(2),CamelCard::T,CamelCard::Num(3),
                CamelCard::K], bid_value: 765},
            CamelCardsHand {
                cards: vec![CamelCard::K,CamelCard::T,CamelCard::J,CamelCard::J,
                CamelCard::T], bid_value: 220},
            CamelCardsHand {
                cards: vec![CamelCard::K,CamelCard::K,CamelCard::Num(6),CamelCard::Num(7),
                CamelCard::Num(7)], bid_value: 28},
            CamelCardsHand {
                cards: vec![CamelCard::T,CamelCard::Num(5),CamelCard::Num(5),CamelCard::J,
                CamelCard::Num(5)], bid_value: 684},
            CamelCardsHand {
                cards: vec![CamelCard::Q,CamelCard::Q,CamelCard::Q,CamelCard::J,
                CamelCard::A], bid_value: 483},
            ]
        );
    }

    #[test]
    fn test_solve_day7p1_1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans,6440);
    }

//    #[test]
//    fn test_solve_day5_p2() {
//    }
//
}
