/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    // Split input into lines
    let input_iter = input.split("\n");

    let count = input_iter
        // Use filter_map instead of map
        // Allows using the ?-operator. Doesn't make a difference because the input is well-formed, but it looks nice
        .filter_map(|line| {
            // Split line to get both ranges
            let (left, right) = line.split_once(',')?;

            // Parse the ranges
            let (l1, l2) = left.split_once('-')?;
            let (r1, r2) = right.split_once('-')?;

            // Parse start and end of the ranges
            let (l1, l2): (u32, u32) = (l1.parse().ok()?, l2.parse().ok()?);
            let (r1, r2): (u32, u32) = (r1.parse().ok()?, r2.parse().ok()?);

            // Check if the left range includes the right range or the other way around
            ((l1 <= r1 && r2 <= l2) || (r1 <= l1 && l2 <= r2)).then_some(())
        })
        .count();

    count.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    // Split input into lines
    let input_iter = input.split("\n");

    let count = input_iter
        // Use filter_map instead of map
        // Allows using the ?-operator. Doesn't make a difference because the input is well-formed, but it looks nice
        .filter_map(|line| {
            // Split line to get both ranges
            let (left, right) = line.split_once(',')?;

            // Parse the ranges
            let (l1, l2) = left.split_once('-')?;
            let (r1, r2) = right.split_once('-')?;

            // Parse start and end of the ranges
            let (l1, l2): (u32, u32) = (l1.parse().ok()?, l2.parse().ok()?);
            let (r1, r2): (u32, u32) = (r1.parse().ok()?, r2.parse().ok()?);

            // Check if the ranges overlap
            (l1 <= r2 && r1 <= l2).then_some(())
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
