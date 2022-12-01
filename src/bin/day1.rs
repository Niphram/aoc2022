fn main() {
    let input = include_str!("inputs/day1.txt");

    println!("Advent of Code 2022 - Day 1");

    // Split input into groups
    let lines = input.split("\n\n");

    // Parse numbers and sum group
    let mut calories: Vec<i32> = lines
        .map(|l| l.split("\n").map(|c| c.parse::<i32>().unwrap()).sum())
        .collect();

    // Sort and reverse vector
    calories.sort();
    calories.reverse();

    let highest = &calories[0];
    let sum_highest_three = &calories[0..3].iter().sum::<i32>();

    println!("Part 1: {}", highest);
    println!("Part 2: {}", sum_highest_three);
}
