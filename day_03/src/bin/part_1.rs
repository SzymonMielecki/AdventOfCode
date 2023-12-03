use std::fs;

use day_03::process_part1;

fn main() {
    //     let input = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";
    let input = fs::read_to_string("./input.txt").unwrap();
    dbg!(process_part1(&input));
}
