use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::newline,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn process_part1(input: &str) -> String {
    let parsed = parse_line(input).expect("should work").1;
    let instructions = parsed.0;
    let hm: HashMap<&str, (&str, &str)> = parsed.1.into_iter().collect();
    let mut current = "AAA";
    let mut counter = 0;

    for ins in instructions.into_iter().cycle() {
        if current == "ZZZ" {
            return counter.to_string();
        }
        counter += 1;
        let selected = hm.get(current).expect("should exist");
        match ins {
            "L" => current = selected.0,
            "R" => current = selected.1,
            _ => (),
        }
    }
    "something went wrong".into()
}
pub fn process_part2(input: &str) -> String {
    let parsed = parse_line(input).expect("should work").1;
    let instructions = parsed.0;
    let hm: HashMap<&str, (&str, &str)> = parsed.1.into_iter().collect();
    let mut currents = hm
        .clone()
        .into_keys()
        .filter(|x| x.ends_with("A"))
        .collect::<Vec<&str>>();

    let mut counter = 0;
    let mut directions = instructions.iter().cycle();

    loop {
        let dir = directions.next().unwrap();
        let end_count = currents.par_iter().filter(|x| x.contains("Z")).count();
        if end_count > 2 {
            println!("{dir}");
            println!("{currents:?}");
            println!("{end_count}");
        }
        if end_count == currents.len() {
            return counter.to_string();
        }
        currents = currents
            .par_iter()
            .map(|x| match *dir {
                "L" => hm.get(x).unwrap().0,
                "R" => hm.get(x).unwrap().1,
                _ => panic!("wtf"),
            })
            .collect();
        counter += 1;
    }
}
fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, Vec<(&str, (&str, &str))>)> {
    separated_pair(
        many1(alt((tag("R"), tag("L")))),
        tuple((newline, newline)),
        separated_list1(
            newline,
            separated_pair(
                take(3usize),
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(take(3usize), tag(", "), take(3usize)),
                    tag(")"),
                ),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let processed = process_part1(&input);
        assert_eq!(processed, "2")
    }
    #[test]
    fn part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let processed = process_part2(&input);
        assert_eq!(processed, "6")
    }
}
