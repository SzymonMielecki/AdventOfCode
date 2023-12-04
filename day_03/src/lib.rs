use std::collections::{HashMap, HashSet};

use nom::{character::complete::anychar, multi::many0, IResult};

// pub fn process_part1(input: &str) -> String {
//     fn map_to_bool_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
//         let rows = grid.len();
//         let cols = grid[0].len();
//
//         let mut result = vec![vec![false; cols]; rows];
//
//         grid.iter().enumerate().for_each(|(i, l)| {
//             l.iter().enumerate().for_each(|(j, c)| {
//                 if !c.is_ascii_digit() && *c != '.' {
//                     (-1..=1).for_each(|x| {
//                         (-1..=1).for_each(|y| {
//                             let ni = i as i32 + x;
//                             let nj = j as i32 + y;
//
//                             if (0..rows).contains(&(ni as usize))
//                                 && (0..cols).contains(&(nj as usize))
//                             {
//                                 result[ni as usize][nj as usize] = true;
//                             }
//                         })
//                     })
//                 }
//             })
//         });
//         result
//     }
//     fn digit_to_num_grid(grid: &Vec<Vec<Option<u32>>>) -> Vec<Vec<Option<u32>>> {
//         let rows = grid.len();
//         let cols = grid[0].len();
//
//         let mut result = grid.clone();
//
//         (0..rows).for_each(|i| {
//             (0..cols).for_each(|j| {
//                 if j > 0 {
//                     if result[i][j - 1].is_some() && result[i][j].is_some() {
//                         result[i][j] = Some(result[i][j - 1].unwrap() * 10 + result[i][j].unwrap())
//                     }
//                 }
//             })
//         });
//         // this checks for 2 digit numbers
//         (0..rows).for_each(|i| {
//             (0..cols).for_each(|j| {
//                 if j < cols - 1 {
//                     if result[i][j].is_some()
//                         && result[i][j + 1].is_some()
//                         && result[i][j + 1].unwrap() > result[i][j].unwrap()
//                     {
//                         result[i][j] = result[i][j + 1];
//                     }
//                 }
//             })
//         });
//         // this checks for 3 digit numbers
//         (0..rows).for_each(|i| {
//             (0..cols).for_each(|j| {
//                 if j < cols - 1 {
//                     if result[i][j].is_some()
//                         && result[i][j + 1].is_some()
//                         && result[i][j + 1].unwrap() > result[i][j].unwrap()
//                     {
//                         result[i][j] = result[i][j + 1];
//                     }
//                 }
//             })
//         });
//         result
//     }
//     fn join_grids(num_grid: &Vec<Vec<Option<u32>>>, valid_grid: &Vec<Vec<bool>>) -> u32 {
//         let rows = num_grid.len();
//         let cols = num_grid[0].len();
//         let mut result = vec![vec![None; cols]; rows];
//         num_grid
//             .iter()
//             .zip(valid_grid.iter())
//             .zip(result.iter_mut())
//             .for_each(|((nl, vl), rl)| {
//                 nl.iter()
//                     .zip(vl.iter())
//                     .zip(rl.iter_mut())
//                     .for_each(|((n, v), r)| {
//                         if *v && n.is_some() {
//                             *r = *n;
//                         }
//                     })
//             });
//
//         let filtered_matrix: Vec<Vec<Option<u32>>> = result
//             .into_iter()
//             .map(|row| {
//                 let mut seen_values = std::collections::HashSet::new();
//                 row.into_iter()
//                     .map(|x| x.filter(|&v| seen_values.insert(v)).map(|v| v))
//                     .collect()
//             })
//             .collect();
//         filtered_matrix
//             .iter()
//             .flat_map(|row| row.iter())
//             .filter_map(|&value| value)
//             .sum()
//     }
//
//     let grid: Vec<Vec<char>> = input
//         .lines()
//         .map(|x| parse_line(x).expect("should work").1)
//         .collect();
//
//     let digit_grid: Vec<Vec<Option<u32>>> = grid
//         .iter()
//         .map(|l| l.iter().map(|c| c.to_digit(10)).collect())
//         .collect();
//
//     let valid_grid = map_to_bool_grid(&grid);
//
//     let num_grid = digit_to_num_grid(&digit_grid);
//
//     let sum = join_grids(&num_grid, &valid_grid);
//
//     sum.to_string()
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    fn join_adjacent_digits(input: &HashMap<Coordinates, Value>) -> HashMap<Coordinates, Value> {
        let mut coords = input.clone();

        for (c, v) in input.iter() {
            if let Value::Digit(v) = v {
                let mut adj: Vec<i32> = vec![];
                for offset in -1..=1 {
                    println!("({},{})", c.x + offset, c.y);
                    if let Some(n) = coords.get_mut(&(Coordinates::new(c.x + offset, c.y))) {
                        if let Value::Number(m) = n {
                            println!("{m:?}");
                            adj.push(*m as i32)
                        } else if let Value::Digit(m) = n {
                            println!("{m:?}");
                            adj.push(*m as i32)
                        }
                    }
                }
                println!("{c:?}, {adj:?}");
                println!("-------------------------");
            }
        }

        coords
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
    let num_coordinates = join_adjacent_digits(&digit_coordinates);

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
