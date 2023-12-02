use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, u32},
    combinator::value,
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        games.push(Game::deconstruct(parse_line(line)))
    }
    games
        .iter()
        .filter(|x| x.valid)
        .map(|x| x.id)
        .sum::<u32>()
        .to_string()
}
pub fn process_part2(input: &str) -> String {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        games.push(Game::deconstruct(parse_line(line)))
    }
    games
        .iter()
        .map(|x| x.green_count * x.blue_count * x.red_count)
        .sum::<u32>()
        .to_string()
}

fn parse_line(input: &str) -> IResult<&str, (u32, Vec<(u32, Color)>)> {
    tuple((
        delimited(tag("Game "), u32, tag(": ")),
        separated_list0(
            alt((tag(", "), tag("; "))),
            separated_pair(
                u32,
                space0,
                alt((
                    value(Color::Blue, tag("blue")),
                    value(Color::Red, tag("red")),
                    value(Color::Green, tag("green")),
                )),
            ),
        ),
    ))(input)
}

#[derive(Debug)]
struct Game {
    id: u32,
    valid: bool,
    green_count: u32,
    blue_count: u32,
    red_count: u32,
}

#[derive(Clone, Debug)]
enum Color {
    Blue,
    Green,
    Red,
}

impl Game {
    pub fn deconstruct(res: IResult<&str, (u32, Vec<(u32, Color)>)>) -> Self {
        if res.is_err() {
            return Game {
                id: 0,
                valid: false,
                green_count: 0,
                blue_count: 0,
                red_count: 0,
            };
        }
        let (_, (id, colors)) = res.unwrap();
        let mut game = Game {
            id,
            valid: true,
            green_count: 0,
            blue_count: 0,
            red_count: 0,
        };
        for (count, color) in colors {
            match color {
                Color::Red => {
                    if count > 12 {
                        game.valid = false;
                    }
                    game.red_count = game.red_count.max(count);
                }
                Color::Green => {
                    if count > 13 {
                        game.valid = false;
                    }
                    game.green_count = game.green_count.max(count);
                }
                Color::Blue => {
                    if count > 14 {
                        game.valid = false;
                    }
                    game.blue_count = game.blue_count.max(count);
                }
            }
        }
        game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let processed = process_part1(&input);
        assert_eq!(processed, "8")
    }
    #[test]
    fn part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let processed = process_part2(&input);
        assert_eq!(processed, "2286")
    }
}
