use std::collections::{HashMap, HashSet};

use nom::{character::complete::anychar, multi::many0, IResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Value {
    Empty,
    Symbol,
    Digit(u32),
    Number(u32),
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}
impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinates { x, y }
    }
    fn get_adjacent(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x + 1, self.y + 1),
        ]
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct SharedCoordinates {
    pub x: Vec<i32>,
    pub y: i32,
}
impl SharedCoordinates {
    pub fn new(x: Vec<i32>, y: i32) -> Self {
        SharedCoordinates { x, y }
    }
    pub fn from(v: Vec<Coordinates>) -> Result<Self, ()> {
        if v.len() == 0 {
            return Err(());
        };
        let y = v[0].y;
        if !v.iter().all(|x| x.y == y) {
            return Err(());
        }
        let mut x = vec![v[0].x];
        for c in v.iter() {
            if x.contains(&(c.x - 1)) || x.contains(&(c.x + 1)) {
                x.push(c.x);
            } else if !x.contains(&(c.x)) {
                return Err(());
            }
        }
        Ok(Self::new(x, y))
    }
    fn contains(&self, other: &SharedCoordinates) -> bool {
        if self.y != other.y {
            return false;
        }
        let self_x = self.x.clone();
        let other_x = other.x.clone();
        for ox in other_x {
            if !self_x.contains(&ox) {
                return false;
            }
        }
        true
    }
}
pub fn process_part1(input: &str) -> String {
    fn map_to_enum(grid: &Vec<Vec<char>>) -> Vec<Vec<Value>> {
        grid.iter()
            .map(|l| {
                l.iter()
                    .map(|c| match c {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Digit(c.to_digit(10).expect("should be digit"))
                        }
                        _ => Value::Symbol,
                    })
                    .collect()
            })
            .collect()
    }

    fn get_symbols_coordinates(grid: &Vec<Vec<Value>>) -> Vec<Coordinates> {
        let mut coords = Vec::new();
        grid.iter().enumerate().for_each(|(y, l)| {
            l.iter().enumerate().for_each(|(x, c)| {
                if *c == Value::Symbol {
                    coords.push(Coordinates::new(x as i32, y as i32));
                }
            });
        });
        coords
    }

    fn get_adjacent_to_symbols(
        symbol_coordinates: &Vec<Coordinates>,
        rows: usize,
        cols: usize,
    ) -> Vec<Coordinates> {
        let mut coords: HashSet<Coordinates> = HashSet::new();
        symbol_coordinates.iter().for_each(|c| {
            c.get_adjacent().iter().for_each(|adj| {
                if (0..cols).contains(&(adj.x as usize)) && (0..rows).contains(&(adj.y as usize)) {
                    coords.insert(*adj);
                }
            })
        });
        coords.into_iter().collect()
    }
    fn get_digit_coordinates(grid: &Vec<Vec<Value>>) -> HashMap<Coordinates, Value> {
        let mut coords = HashMap::new();
        grid.iter().enumerate().for_each(|(y, l)| {
            l.iter().enumerate().for_each(|(x, c)| {
                if let Value::Digit(_) = c {
                    coords.insert(Coordinates::new(x as i32, y as i32), *c);
                }
            });
        });
        coords
    }

    fn join_adjacent_digits(
        input: &HashMap<Coordinates, Value>,
    ) -> HashMap<SharedCoordinates, Value> {
        let mut res = HashMap::new();
        for (c, value) in input.iter() {
            let mut this_num: Vec<(Value, Coordinates)> = vec![];
            if let Value::Digit(v) = value {
                for offset in -1..=1 {
                    if let Some(adj) = input.get(&Coordinates::new(c.x + offset, c.y)) {
                        this_num.push((*adj, Coordinates::new(c.x + offset, c.y)))
                    }
                }
            }
            println!("{this_num:?}");
            this_num.sort_by(|a, b| a.1.x.cmp(&b.1.x));
            let this_num_val: Value = Value::Number(
                this_num
                    .iter()
                    .filter_map(|x| {
                        if let Value::Digit(n) = x.0 {
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .fold(0, |acc, x| acc * 10 + x),
            );
            if let Ok(n) = SharedCoordinates::from(this_num.iter().map(|(v, c)| *c).collect()) {
                res.insert(n, this_num_val);
            }

            println!("-------------------------");
        }
        res
    }
    fn filter_adjacent_digits(hm: &mut HashMap<SharedCoordinates, Value>) {
        let binding = hm.clone();
        let keys = binding.keys();
        for k in keys.clone() {
            if keys.clone().into_iter().any(|x| x != k && x.contains(k)) {
                hm.remove(k);
            }
        }
    }
    let char_grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| parse_line(l).expect("should exist").1)
        .collect();

    let rows = char_grid.len();
    let cols = char_grid[0].len();

    let enum_grid: Vec<Vec<Value>> = map_to_enum(&char_grid);

    let symbol_coordinates: Vec<Coordinates> = get_symbols_coordinates(&enum_grid);
    let symbol_adjacent: Vec<Coordinates> =
        get_adjacent_to_symbols(&symbol_coordinates, rows, cols);

    let digit_coordinates = get_digit_coordinates(&enum_grid);
    let mut num_coordinates = join_adjacent_digits(&digit_coordinates);
    dbg!(&num_coordinates);
    filter_adjacent_digits(&mut num_coordinates);
    dbg!(&num_coordinates);
    "".into()
}
fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    many0(anychar)(input)
}
pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let processed = process_part1(&input);
        assert_eq!(processed, "4361")
    }
    #[test]
    fn part2() {
        let input = "";
        let processed = process_part2(&input);
        assert_eq!(processed, "")
    }
}
