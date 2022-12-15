use std::{collections::HashSet, ops::RangeTo, thread::panicking};

type Pos = (i64, i64);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon_pos: Pos,
}

impl Sensor {
    const fn radius(&self) -> u64 {
        let (sens_x, sens_y) = self.pos;
        let (beac_x, beac_y) = self.beacon_pos;

        sens_x.abs_diff(beac_x) + sens_y.abs_diff(beac_y)
    }

    const fn dist(&self, pos: Pos) -> u64 {
        let (sens_x, sens_y) = self.pos;
        sens_x.abs_diff(pos.0) + sens_y.abs_diff(pos.1)
    }

    fn covers(&self, pos: Pos) -> bool {
        let radius = self.radius();

        let (sens_x, sens_y) = self.pos;
        let distance = sens_x.abs_diff(pos.0) + sens_y.abs_diff(pos.1);

        distance <= radius
    }

    fn covers_row(&self, row: i64) -> std::ops::Range<i64> {
        let radius = self.radius();

        let (sens_x, sens_y) = self.pos;
        let y_dist = sens_y.abs_diff(row);

        let coverage = radius.abs_diff(y_dist) as i64;

        if y_dist < radius {
            (sens_x - coverage)..(sens_x + coverage)
        } else {
            0..0
        }
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.strip_prefix("Sensor at x=")?;
            let (x_sensor, line) = line.split_once(", y=")?;
            let (y_sensor, line) = line.split_once(": closest beacon is at x=")?;
            let (x_beacon, y_beacon) = line.split_once(", y=")?;

            Some(Sensor {
                pos: (x_sensor.parse().ok()?, y_sensor.parse().ok()?),
                beacon_pos: (x_beacon.parse().ok()?, y_beacon.parse().ok()?),
            })
        })
        .collect()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let sensors = parse_input(input);

    let mut covered = HashSet::new();

    for sensor in &sensors {
        for i in sensor.covers_row(2000000) {
            covered.insert(i);
        }
    }

    covered.len().to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let sensors = parse_input(input);

    let mut found = None;

    'outer: for y in 0..=4000000 {
        let mut x = 0;
        while x <= 4000000 {
            let new_x = sensors
                .iter()
                .filter_map(|sensor| sensor.covers((x, y)).then_some(sensor.covers_row(y).end))
                .max();

            if let Some(new_x) = new_x {
                x = new_x + 1;
            } else {
                found = Some(x * 4000000 + y);
                break 'outer;
            }
        }
    }

    found.unwrap().to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 15");

    let part_1 = part_1(input);
    println!("Part 1: {part_1}");

    let part_2 = part_2(input);
    println!("Part 2: {part_2}");
}
