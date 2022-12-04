use std::collections::BTreeSet;

/// Maps a-zA-Z to the range 1-52
fn char_to_priority(c: char) -> u32 {
    match c {
        c @ 'a'..='z' => c as u32 - 'a' as u32 + 1,
        c @ 'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    // Split input into backpacks
    let backpack_iter = input.split("\n");

    let priorities_sum = backpack_iter
        .map(|l| {
            // Split backpack at middle
            let (left, right) = l.split_at(l.len() / 2);

            // Collect chars into hashsets
            let left_set = left.chars().collect::<BTreeSet<_>>();
            let right_set = right.chars().collect::<BTreeSet<_>>();

            // Intersect both sets and take the result
            // Only one item will be in here
            let item = left_set.intersection(&right_set).next().unwrap();

            // Get priority
            char_to_priority(*item)
        })
        // Sum all priorities
        .sum::<u32>();

    priorities_sum.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    // Split input into lines and collect into vector
    let lines = input.split("\n").collect::<Vec<_>>();

    // Create chunks-iterator
    let lines_iter = lines.chunks(3);

    let priorities_sum = lines_iter
        .map(|group| {
            let badge_set = group
                .iter()
                // Collect backback into hashset
                .map(|backpack| backpack.chars().collect::<BTreeSet<_>>())
                // intersect all backpacks
                .reduce(|acc, backpack| {
                    acc.intersection(&backpack)
                        .copied()
                        .collect::<BTreeSet<_>>()
                })
                .unwrap();

            // Only one item will be in here
            let badge = badge_set.iter().next().unwrap();

            // Get priority
            char_to_priority(*badge)
        })
        // Sum all priorities
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
