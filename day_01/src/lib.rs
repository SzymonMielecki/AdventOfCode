use nom::{branch::alt, bytes::streaming::tag, combinator::value, IResult};

pub fn process_part1(input: &str) -> String {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let vec: Vec<u32> = line
            .chars()
            .filter(|x| x.is_digit(10))
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        sum += vec.first().unwrap() * 10 + vec.last().unwrap()
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    fn get_valid_substring(input: &str) -> String {
        fn get_valid_start_substring(input: &str) -> String {
            if starts_with_correct(input) {
                return input.into();
            }
            for i in 1..input.len() {
                if starts_with_correct(&input[i..]) {
                    return input[i..].into();
                }
            }
            "".into()
        }
        fn get_valid_end_substring(input: &str) -> String {
            let input: String = input.chars().rev().collect();
            if starts_with_correct(&input) {
                return input.chars().rev().collect();
            }
            for i in 1..input.len() {
                if starts_with_correct(&input[i..]) {
                    return input[i..].chars().rev().collect();
                }
            }
            "".into()
        }
        let stripped_start = get_valid_start_substring(input);
        get_valid_end_substring(&stripped_start)
    }
    let mut sum: u32 = 0;
    for line in input.lines() {
        let line = get_valid_substring(line);
        let (_, first) = parse_num_forward(&line).unwrap();
        let rev_line: String = line.chars().rev().collect();
        let (_, last) = parse_num_backward(&rev_line).unwrap();
        let res = first.parse::<u32>().unwrap() * 10 + last.parse::<u32>().unwrap();
        sum += res
    }
    sum.to_string()
}
fn starts_with_correct(s: &str) -> bool {
    let arr = &[
        "one", "eno", "1", "owt", "two", "2", "eerht", "three", "3", "ruof", "four", "4", "evif",
        "five", "5", "xis", "six", "6", "seven", "neves", "7", "eight", "thgie", "8", "nine",
        "enin", "9", "zero", "orez", "0",
    ];

    for &substring in arr {
        if s.starts_with(substring) {
            return true;
        }
    }
    false
}
fn parse_num_forward(input: &str) -> IResult<&str, &str> {
    alt((
        value("1", tag("one")),
        value("2", tag("two")),
        value("3", tag("three")),
        value("4", tag("four")),
        value("5", tag("five")),
        value("6", tag("six")),
        value("7", tag("seven")),
        value("8", tag("eight")),
        value("9", tag("nine")),
        value("0", tag("zero")),
        tag("1"),
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
        tag("0"),
    ))(input)
}
fn parse_num_backward(input: &str) -> IResult<&str, &str> {
    alt((
        value("1", tag("eno")),
        value("2", tag("owt")),
        value("3", tag("eerht")),
        value("4", tag("ruof")),
        value("5", tag("evif")),
        value("6", tag("xis")),
        value("7", tag("neves")),
        value("8", tag("thgie")),
        value("9", tag("enin")),
        value("0", tag("orez")),
        tag("1"),
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
        tag("0"),
    ))(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let processed = process_part1(&input);
        assert_eq!(processed, "142")
    }
    #[test]
    fn part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let processed = process_part2(&input);
        assert_eq!(processed, "281")
    }
}
