use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space1, u32},
    combinator::value,
    multi::{many0, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Card {
    A,
    K,
    Q,
    J,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
impl Card {
    fn get_inner_value(&self) -> u32 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
}

#[derive(Debug)]
enum Variant {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKinds,
    TwoPair,
    OnePair,
    OnePairExtra,
    HighCard,
}
impl Variant {
    pub fn new(cards: Vec<Card>) -> Variant {
        let mut seen: HashMap<Card, u32> = HashMap::new();
        for card in cards.clone() {
            *seen.entry(card).or_insert(0) += 1;
        }
        let mut tmp_var = match seen.len() {
            1 => Variant::FiveOfAKind,
            5 => Variant::HighCard,
            4 => Variant::OnePair,
            3 => match seen.values().any(|&x| x == 3) {
                true => Variant::ThreeOfAKinds,
                false => Variant::TwoPair,
            },
            2 => match seen.values().any(|&x| x == 4) {
                true => Variant::FourOfAKind,
                false => Variant::FullHouse,
            },
            _ => panic!(),
        };
        for _ in 0..cards
            .clone()
            .into_iter()
            .filter(|x| x == &Card::J)
            .collect::<Vec<Card>>()
            .len()
        {
            match tmp_var {
                Variant::HighCard => tmp_var = Variant::OnePair,
                Variant::OnePair => {
                    if seen.get(&Card::J) != Some(&2) {
                        tmp_var = Variant::ThreeOfAKinds
                    } else {
                        tmp_var = Variant::OnePairExtra
                    }
                }
                Variant::OnePairExtra => tmp_var = Variant::ThreeOfAKinds,
                Variant::TwoPair => tmp_var = Variant::FullHouse,
                Variant::ThreeOfAKinds => tmp_var = Variant::FourOfAKind,
                Variant::FourOfAKind => {
                    if seen.len() == 2 {
                        tmp_var = Variant::FiveOfAKind
                    }
                }
                Variant::FullHouse => tmp_var = Variant::FourOfAKind,
                Variant::FiveOfAKind => (),
            }
        }
        tmp_var
    }
    fn get_outer_value(&self) -> u32 {
        match self {
            Variant::FiveOfAKind => 6,
            Variant::FourOfAKind => 5,
            Variant::FullHouse => 4,
            Variant::ThreeOfAKinds => 3,
            Variant::TwoPair => 2,
            Variant::OnePair => 1,
            Variant::OnePairExtra => 1,
            Variant::HighCard => 0,
        }
    }
}
impl Hand {
    fn compare(&self, other: &Hand) -> Ordering {
        if self.variant.get_outer_value() != other.variant.get_outer_value() {
            return self
                .variant
                .get_outer_value()
                .cmp(&other.variant.get_outer_value());
        }
        for (s, o) in self.cards.iter().zip(other.cards.iter()) {
            if s.get_inner_value() != o.get_inner_value() {
                return s.get_inner_value().cmp(&o.get_inner_value());
            }
        }
        return Ordering::Equal;
    }
    pub fn new(cards: Vec<Card>, bid: u32) -> Hand {
        if cards.len() != 5 {
            panic!()
        }
        Hand {
            cards: cards.clone(),
            variant: Variant::new(cards),
            bid,
        }
    }
}
#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    variant: Variant,
    bid: u32,
}
fn parse_line(input: &str) -> IResult<&str, Vec<(Vec<Card>, u32)>> {
    separated_list1(
        newline,
        separated_pair(
            many0(alt((
                value(Card::Two, tag("2")),
                value(Card::Three, tag("3")),
                value(Card::Four, tag("4")),
                value(Card::Five, tag("5")),
                value(Card::Six, tag("6")),
                value(Card::Seven, tag("7")),
                value(Card::Eight, tag("8")),
                value(Card::Nine, tag("9")),
                value(Card::Ten, tag("T")),
                value(Card::J, tag("J")),
                value(Card::Q, tag("Q")),
                value(Card::K, tag("K")),
                value(Card::A, tag("A")),
            ))),
            space1,
            u32,
        ),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let parsed = parse_line(input).unwrap().1;
    let mut hands = parsed
        .into_iter()
        .map(|x| Hand::new(x.0, x.1))
        .collect::<Vec<Hand>>();
    hands.sort_by(|x, y| x.compare(&y));
    dbg!(hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1, h))
        .collect::<Vec<(usize, Hand)>>())
    .into_iter()
    .fold(0, |acc, (i, h)| acc + h.bid * i as u32)
    .to_string()
}
pub fn process_part2(input: &str) -> String {
    let parsed = parse_line(input).unwrap().1;
    let mut hands = parsed
        .into_iter()
        .map(|x| Hand::new(x.0, x.1))
        .collect::<Vec<Hand>>();
    hands.sort_by(|x, y| x.compare(&y));
    dbg!(hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1, h))
        .collect::<Vec<(usize, Hand)>>())
    .into_iter()
    .fold(0, |acc, (i, h)| acc + h.bid * i as u32)
    .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let processed = process_part1(&input);
        // assert_eq!(processed, "6440")
    }
    #[test]
    fn part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let processed = process_part2(&input);
        assert_eq!(processed, "5905")
    }
}
