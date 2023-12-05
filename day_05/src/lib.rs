use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u64},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

struct Location {
    options: Vec<Row>,
}

impl Location {
    pub fn new(vv: Vec<Vec<u64>>) -> Self {
        Location {
            options: vv.into_iter().map(|v| Row::new(v[0], v[1], v[2])).collect(),
        }
    }
    fn get_output(&self, value: u64) -> u64 {
        self.options
            .iter()
            .filter_map(|opt| opt.get_output(value))
            .min()
            .unwrap_or(value)
    }
}
struct Row {
    dest_start: u64,
    source_range: Range<u64>,
}

impl Row {
    pub fn new(dest_start: u64, source_start: u64, len: u64) -> Self {
        Row {
            dest_start,
            source_range: (source_start..source_start + len),
        }
    }
    fn get_output(&self, value: u64) -> Option<u64> {
        if !self.source_range.contains(&value) {
            return None;
        }
        Some(&self.dest_start + value - self.source_range.start)
    }
}
pub fn process_part1(input: &str) -> String {
    let processed = parse_file(&input).expect("should work").1;
    let (seeds, s2s, s2f, f2w, w2l, l2t, t2h, h2l) = processed;
    let seed_to_soil = Location::new(s2s);
    let soil_to_fertilizer = Location::new(s2f);
    let fertilizer_to_water = Location::new(f2w);
    let water_to_light = Location::new(w2l);
    let light_to_temperature = Location::new(l2t);
    let temperature_to_humidity = Location::new(t2h);
    let humidity_to_location = Location::new(h2l);
    seeds
        .into_iter()
        .map(|seed| {
            humidity_to_location.get_output(temperature_to_humidity.get_output(
                light_to_temperature.get_output(
                    water_to_light.get_output(
                        fertilizer_to_water.get_output(
                            soil_to_fertilizer.get_output(seed_to_soil.get_output(seed)),
                        ),
                    ),
                ),
            ))
        })
        .min()
        .unwrap()
        .to_string()
}
pub fn process_part2(input: &str) -> String {
    input.into()
}

fn parse_file(
    input: &str,
) -> IResult<
    &str,
    (
        Vec<u64>,
        Vec<Vec<u64>>,
        Vec<Vec<u64>>,
        Vec<Vec<u64>>,
        Vec<Vec<u64>>,
        Vec<Vec<u64>>,
        Vec<Vec<u64>>,
        Vec<Vec<u64>>,
    ),
> {
    tuple((
        delimited(
            tuple((tag("seeds:"), space1)),
            separated_list0(space1, u64),
            tuple((newline, newline)),
        ),
        delimited(
            tuple((tag("seed-to-soil map:"), newline)),
            separated_list1(newline, separated_list1(space1, u64)),
            tuple((newline, newline)),
        ),
        delimited(
            tuple((tag("soil-to-fertilizer map:"), newline)),
            separated_list1(newline, separated_list1(space1, u64)),
            tuple((newline, newline)),
        ),
        delimited(
            tuple((tag("fertilizer-to-water map:"), newline)),
            separated_list1(newline, separated_list1(space1, u64)),
            tuple((newline, newline)),
        ),
        delimited(
            tuple((tag("water-to-light map:"), newline)),
            separated_list1(newline, separated_list1(space1, u64)),
            tuple((newline, newline)),
        ),
        delimited(
            tuple((tag("light-to-temperature map:"), newline)),
            separated_list1(newline, separated_list1(space1, u64)),
            tuple((newline, newline)),
        ),
        delimited(
            tuple((tag("temperature-to-humidity map:"), newline)),
            separated_list1(newline, separated_list1(space1, u64)),
            tuple((newline, newline)),
        ),
        delimited(
            tuple((tag("humidity-to-location map:"), newline)),
            separated_list1(newline, separated_list1(space1, u64)),
            space0,
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let processed = process_part1(&input);
        assert_eq!(processed, "35")
    }
    #[test]
    fn part2() {
        let input = "";
        let processed = process_part2(&input);
        assert_eq!(processed, "")
    }
}
