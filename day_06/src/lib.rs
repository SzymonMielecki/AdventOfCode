use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, space1, u64},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Race {
    length: u64,
    record: u64,
    options: u64,
}

impl Race {
    pub fn new(length: u64, record: u64) -> Self {
        Race {
            length,
            record,
            options: (1..length)
                .map(|l| l * (length - l))
                .filter(|x| x > &record)
                .count() as u64,
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let parsed = parse_line(input).expect("should work").1;
    parsed
        .0
        .iter()
        .zip(parsed.1.iter())
        .map(|x| Race::new(*x.0, *x.1))
        .fold(1, |acc, x| acc * x.options)
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let parsed = parse_line(input).expect("should work").1;
    Race::new(
        parsed
            .0
            .into_iter()
            .fold(String::new(), |acc, num| acc + &num.to_string())
            .parse::<u64>()
            .expect("should work"),
        parsed
            .1
            .into_iter()
            .fold(String::new(), |acc, num| acc + &num.to_string())
            .parse::<u64>()
            .expect("should work"),
    )
    .options
    .to_string()
}

fn parse_line(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    tuple((
        delimited(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, u64),
            newline,
        ),
        preceded(
            tuple((tag("Distance:"), space1)),
            separated_list1(space1, u64),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let processed = process_part1(&input);
        assert_eq!(processed, "288")
    }
    #[test]
    fn part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let processed = process_part2(&input);
        assert_eq!(processed, "71503")
    }
}
