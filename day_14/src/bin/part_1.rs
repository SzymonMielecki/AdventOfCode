use std::fs;

use day_14::process_part1;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    dbg!(process_part1(&input));
}
