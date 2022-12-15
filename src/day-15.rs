use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::read_to_string,
    ops::RangeInclusive,
};

const SAMPLE_ROW: i64 = 2_000_000;
const MAX_COORD: i64 = 4_000_000;

fn main() {
    let input = read_to_string("inputs/day-15.txt").expect("missing input file");
    let (sensors, beacons) = parse_input(input);
    let ranges = coverage_at_row(&sensors, SAMPLE_ROW);
    let beacons_at_row = beacons.iter().filter(|(_, y)| *y == SAMPLE_ROW).count();
    let count_cov =
        ranges.iter().map(|r| r.end() - r.start() + 1).sum::<i64>() - beacons_at_row as i64;
    println!("Part 1: {}", count_cov);

    let (x, y) = find_beacon(&sensors, MAX_COORD);
    println!("Part 2: {}", (x * MAX_COORD + y));
}

type Point = (i64, i64);

#[derive(Debug)]
struct Sensor {
    point: Point,
    reach: i64,
}

type Beacon = Point;

impl Sensor {
    fn coverage_at_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let (sx, sy) = self.point;
        let spread = self.reach - (y - sy).abs();
        if spread >= 0 {
            Some(sx - spread..=sx + spread)
        } else {
            None
        }
    }
}


fn find_beacon(sensors: &[Sensor], max: i64) -> Beacon {
    for y in 0..=max {
        let ranges = coverage_at_row(sensors, y as i64);
        if ranges.len() > 1 {
            return (ranges[0].end() + 1, y)
        }
    }
    panic!("no beacon found! real panic")
}

fn coverage_at_row(sensors: &[Sensor], row: i64) -> Vec<RangeInclusive<i64>> {
    let ranges = sensors
        .iter()
        .filter_map(|s| s.coverage_at_y(row))
        .collect();
    reduce_ranges(ranges)
}

fn reduce_ranges(mut ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    let mut prev_len = 0;
    while ranges.len() != prev_len {
        prev_len = ranges.len();
        ranges = ranges.into_iter().fold(vec![], |acc, range| {
            let mut reduced = vec![];
            let mut found = false;
            for other in acc.into_iter() {
                if range.contains(other.start())
                    || range.contains(other.end())
                    || other.contains(range.start())
                    || other.contains(range.end())
                {
                    reduced
                        .push(*min(range.start(), other.start())..=*max(range.end(), other.end()));
                    found = true;
                } else {
                    reduced.push(other);
                }
            }
            if !found {
                reduced.push(range);
            }
            reduced
        });
    }
    ranges
}

fn parse_input(input: String) -> (Vec<Sensor>, HashSet<Beacon>) {
    input.lines().map(parse_pair).unzip()
}

fn parse_pair(input: &str) -> (Sensor, Beacon) {
    let mut parts = input.split(": ");
    let mut sensor_parts = parts
        .next()
        .expect("should have sensor part")
        .strip_prefix("Sensor at ")
        .expect("should have prefix")
        .split(", ");
    let mut beacon_parts = parts
        .next()
        .expect("should have beacon part")
        .strip_prefix("closest beacon is at ")
        .expect("should have prefix")
        .split(", ");

    let sx: i64 = sensor_parts
        .next()
        .expect("should have x")
        .strip_prefix("x=")
        .expect("should have prefix")
        .parse()
        .expect("must be num");
    let sy: i64 = sensor_parts
        .next()
        .expect("should have y")
        .strip_prefix("y=")
        .expect("should have prefix")
        .parse()
        .expect("must be num");

    let bx: i64 = beacon_parts
        .next()
        .expect("should have x")
        .strip_prefix("x=")
        .expect("should have prefix")
        .parse()
        .expect("must be num");
    let by: i64 = beacon_parts
        .next()
        .expect("should have y")
        .strip_prefix("y=")
        .expect("should have prefix")
        .parse()
        .expect("must be num");
    (
        Sensor {
            point: (sx, sy),
            reach: ((bx - sx).abs() + (by - sy).abs()),
        },
        (bx, by),
    )
}
