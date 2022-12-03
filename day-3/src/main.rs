use std::collections::HashSet;

/// Compute the solution to part 1
pub fn part_1(input: &str) -> String {
    let lines_iter = input.split("\n");

    let priorities_sum = lines_iter
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);

            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();

            a.intersection(&b)
                .map(|c| match c {
                    'a'..='z' => *c as u32 - 96u32,
                    'A'..='Z' => *c as u32 - 38u32,
                    _ => unreachable!(),
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    priorities_sum.to_string()
}

/// Compute the solution to part 2
pub fn part_2(input: &str) -> String {
    let lines = input.split("\n").collect::<Vec<_>>();
    let lines_iter = lines.chunks(3);

    let priorities_sum = lines_iter
        .map(|l| {
            let badge = l
                .iter()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .reduce(|acc, item| acc.intersection(&item).copied().collect::<HashSet<_>>())
                .unwrap();

            badge
                .iter()
                .map(|c| match c {
                    'a'..='z' => *c as u32 - 96u32,
                    'A'..='Z' => *c as u32 - 38u32,
                    _ => unreachable!(),
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    priorities_sum.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 3");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
