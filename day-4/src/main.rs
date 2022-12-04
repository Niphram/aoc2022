use std::{collections::BTreeSet, ops::RangeBounds};

/// Compute the solution to part 1
pub fn part_1(input: &str) -> String {
    let input_iter = input.split("\n");

    let count = input_iter
        .filter(|line| {
            let (left, right) = line.split_once(',').unwrap();

            let left_range = left.split_once('-').unwrap();
            let right_range = right.split_once('-').unwrap();

            let left_range = (
                left_range.0.parse::<u32>().unwrap(),
                left_range.1.parse::<u32>().unwrap(),
            );
            let right_range = (
                right_range.0.parse::<u32>().unwrap(),
                right_range.1.parse::<u32>().unwrap(),
            );

            (left_range.0 <= right_range.0 && left_range.1 >= right_range.1)
                || (right_range.0 <= left_range.0 && right_range.1 >= left_range.1)
        })
        .count();

    count.to_string()
}

/// Compute the solution to part 2
pub fn part_2(input: &str) -> String {
    let input_iter = input.split("\n");

    let count = input_iter
        .filter(|line| {
            let (left, right) = line.split_once(',').unwrap();

            let left_range = left.split_once('-').unwrap();
            let right_range = right.split_once('-').unwrap();

            let left_range = (
                left_range.0.parse::<u32>().unwrap(),
                left_range.1.parse::<u32>().unwrap(),
            );
            let right_range = (
                right_range.0.parse::<u32>().unwrap(),
                right_range.1.parse::<u32>().unwrap(),
            );

            let l = left_range.0..=left_range.1;
            let r = right_range.0..=right_range.1;

            l.contains(r.start())
                || l.contains(r.end())
                || r.contains(l.start())
                || r.contains(l.end())
        })
        .count();

    count.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 4");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
