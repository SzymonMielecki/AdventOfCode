use std::fs;

use day_08::process_part2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    dbg!(process_part2(&input));
}
