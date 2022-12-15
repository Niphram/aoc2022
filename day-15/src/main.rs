use std::collections::HashSet;

#[derive(Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    /// Calculate the manhattan distance between two positions
    fn dis(&self, other: &Pos) -> u64 {
        i64::abs_diff(self.x, other.x) + i64::abs_diff(self.y, other.y)
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
    radius: u64,
}

impl Sensor {
    fn new(sensor_pos: Pos, beacon_pos: Pos) -> Self {
        let radius = Pos::dis(&sensor_pos, &beacon_pos);

        Sensor {
            pos: sensor_pos,
            beacon: beacon_pos,
            radius,
        }
    }

    /// Checks if the position is covered by this sensor
    fn covers(&self, pos: &Pos) -> bool {
        let distance = self.pos.dis(pos);
        distance <= self.radius
    }

    /// Returns a range that describes the x-values this sensor covers in that row
    /// If the sensor doesn't cover the row at all, None is returned
    fn covers_row(&self, row: i64) -> Option<std::ops::RangeInclusive<i64>> {
        let Pos { x, y } = self.pos;
        let y_dist = y.abs_diff(row);

        // The value that can be added/subtracted from the x-position
        let coverage = self.radius.abs_diff(y_dist) as i64;

        // Only return range if the sensor actually covers the row
        (y_dist <= self.radius).then_some((x - coverage)..=(x + coverage))
    }
}

/// Parse input, nothing special
fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.strip_prefix("Sensor at x=")?;
            let (x_sensor, line) = line.split_once(", y=")?;
            let (y_sensor, line) = line.split_once(": closest beacon is at x=")?;
            let (x_beacon, y_beacon) = line.split_once(", y=")?;

            let sensor_pos = Pos::new(x_sensor.parse().ok()?, y_sensor.parse().ok()?);
            let beacon_pos = Pos::new(x_beacon.parse().ok()?, y_beacon.parse().ok()?);

            Some(Sensor::new(sensor_pos, beacon_pos))
        })
        .collect()
}

/// Compute the solution to part 1
fn part_1(row: i64, input: &str) -> String {
    let sensors = parse_input(input);

    // Use a hashset to keep track of positions. Probably not the fastest but it works
    let mut covered = HashSet::new();

    // For every sensor add the covered positions in the row to a hashset
    for sensor in &sensors {
        if let Some(range) = sensor.covers_row(row) {
            for x in range {
                covered.insert(x);
            }
        }
    }

    // Remove beacons from the covered set
    // This was missing in my solution before but the answer was right by coincidence...
    for sensor in &sensors {
        if sensor.beacon.y == row {
            covered.remove(&sensor.beacon.x);
        }
    }

    // Length of the set is the answer
    covered.len().to_string()
}

/// Compute the solution to part 2
fn part_2(range: i64, input: &str) -> String {
    let sensors = parse_input(input);

    let mut result = 0;

    // Check every position by scanning from left to right and top to bottom
    'outer: for y in 0..=range {
        let mut x = 0;
        while x <= range {
            // For each sensor:
            let new_x = sensors
                .iter()
                // Check which sensors cover the given position
                .filter(|sensor| sensor.covers(&Pos::new(x, y)))
                // Get the range the sensor covers
                .filter_map(|sensor| sensor.covers_row(y))
                // Only consider the ends of the ranges
                .map(|range| *range.end())
                // Get the highest one
                .max();

            if let Some(new_x) = new_x {
                // If the above returns some value, we can safely move x ahead, because all those positions are covered
                x = new_x + 1;
            } else {
                // Otherwise we have found an uncovered position and can stop
                result = x * 4000000 + y;
                break 'outer;
            }
        }
    }

    result.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 15");

    let part_1_res = part_1(2000000, input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(4000000, input);
    println!("Part 2: {part_2_res}");
}
