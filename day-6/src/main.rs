use std::collections::HashSet;

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let idx = input
        .as_bytes()
        .windows(4)
        .position(|w| w.into_iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        + 4;

    idx.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let idx = input
        .as_bytes()
        .windows(14)
        .position(|w| w.into_iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        + 14;

    idx.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 6");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
