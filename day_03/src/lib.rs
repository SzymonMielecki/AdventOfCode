use nom::{character::complete::anychar, multi::many0, IResult};

pub fn process_part1(input: &str) -> String {
    fn map_to_bool_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut result = vec![vec![false; cols]; rows];

        grid.iter().enumerate().for_each(|(i, l)| {
            l.iter().enumerate().for_each(|(j, c)| {
                if !c.is_ascii_digit() && *c != '.' {
                    (-1..=1).for_each(|x| {
                        (-1..=1).for_each(|y| {
                            let ni = i as i32 + x;
                            let nj = j as i32 + y;

                            if (0..rows).contains(&(ni as usize))
                                && (0..cols).contains(&(nj as usize))
                            {
                                result[ni as usize][nj as usize] = true;
                            }
                        })
                    })
                }
            })
        });
        result
    }
    fn digit_to_num_grid(grid: &Vec<Vec<Option<u32>>>) -> Vec<Vec<Option<u32>>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut result = grid.clone();
        for i in 0..rows {
            for j in 0..cols {
                if j > 0 {
                    if result[i][j - 1].is_some() && result[i][j].is_some() {
                        result[i][j] = Some(result[i][j - 1].unwrap() * 10 + result[i][j].unwrap())
                    }
                }
            }
        }
        // this checks for 2 digit numbers
        for i in 0..rows {
            for j in 0..cols {
                if j < cols - 1 {
                    if result[i][j].is_some()
                        && result[i][j + 1].is_some()
                        && result[i][j + 1].unwrap() > result[i][j].unwrap()
                    {
                        result[i][j] = result[i][j + 1];
                    }
                }
            }
        }
        // this checks for 3 digit numbers
        for i in 0..rows {
            for j in 0..cols {
                if j < cols - 1 {
                    if result[i][j].is_some()
                        && result[i][j + 1].is_some()
                        && result[i][j + 1].unwrap() > result[i][j].unwrap()
                    {
                        result[i][j] = result[i][j + 1];
                    }
                }
            }
        }
        result
    }
    fn join_grids(num_grid: &Vec<Vec<Option<u32>>>, valid_grid: &Vec<Vec<bool>>) -> u32 {
        let rows = num_grid.len();
        let cols = num_grid[0].len();
        let mut result = vec![vec![None; cols]; rows];
        num_grid
            .iter()
            .zip(valid_grid.iter())
            .zip(result.iter_mut())
            .for_each(|((nl, vl), rl)| {
                nl.iter()
                    .zip(vl.iter())
                    .zip(rl.iter_mut())
                    .for_each(|((n, v), r)| {
                        if *v && n.is_some() {
                            *r = *n;
                        }
                    })
            });

        let filtered_matrix: Vec<Vec<Option<u32>>> = result
            .into_iter()
            .map(|row| {
                let mut seen_values = std::collections::HashSet::new();
                row.into_iter()
                    .map(|x| x.filter(|&v| seen_values.insert(v)).map(|v| v))
                    .collect()
            })
            .collect();
        filtered_matrix
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|&value| value)
            .sum()
    }

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|x| parse_line(x).expect("should work").1)
        .collect();

    let digit_grid: Vec<Vec<Option<u32>>> = grid
        .iter()
        .map(|l| l.iter().map(|c| c.to_digit(10)).collect())
        .collect();

    let valid_grid = map_to_bool_grid(&grid);

    let num_grid = digit_to_num_grid(&digit_grid);

    let sum = join_grids(&num_grid, &valid_grid);

    sum.to_string()
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
