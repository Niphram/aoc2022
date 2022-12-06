fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 1");

    // Split input into groups seperated by empty lines
    let grouped_input = input.split("\n\n");

    // Parse and sum for every group
    let mut sums = grouped_input
        .map(|l| {
            // Parse all strings to numbers
            let numbers = l.lines().flat_map(str::parse::<u32>);

            // Return sum
            numbers.sum()
        })
        .collect::<Vec<_>>();

    // Sort and reverse vector
    sums.sort_unstable();
    sums.reverse();

    // Highest will be at the front
    let highest = sums[0];

    // Slice the first three and sum them
    let sum_highest_three: u32 = sums[0..3].iter().sum();

    println!("Part 1: {}", highest);
    println!("Part 2: {}", sum_highest_three);
}
