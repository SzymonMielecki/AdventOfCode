use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Game {
    id: u32,
    winnging_nums: Vec<u32>,
    my_nums: Vec<u32>,
    mutual: Vec<u32>,
}
impl Game {
    pub fn deconstructed(res: IResult<&str, (u32, (Vec<u32>, Vec<u32>))>) -> Self {
        let (_, (id, (winnging_nums, my_nums))) = res.expect("should work");
        let set_my: HashSet<u32> = my_nums.clone().into_iter().collect();
        let mutual = winnging_nums
            .clone()
            .into_iter()
            .filter(|x| set_my.contains(x))
            .collect();
        Game {
            id,
            winnging_nums,
            my_nums,
            mutual,
        }
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|l| Game::deconstructed(parse_line(l)))
        .filter(|x| x.mutual.len() > 0)
        .map(|x| (2 as usize).pow(x.mutual.len() as u32 - 1))
        .sum::<usize>()
        .to_string()
}

fn parse_line(input: &str) -> IResult<&str, (u32, (Vec<u32>, Vec<u32>))> {
    tuple((
        delimited(tuple((tag("Card"), space1)), u32, tuple((tag(":"), space1))),
        separated_pair(
            separated_list0(space1, u32),
            tuple((space1, tag("|"), space1)),
            separated_list0(space1, u32),
        ),
    ))(input)
}
pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let processed = process_part1(&input);
        assert_eq!(processed, "13")
    }
    #[test]
    fn part2() {
        let input = "";
        let processed = process_part2(&input);
        assert_eq!(processed, "")
    }
}
