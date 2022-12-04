/// Compute the solution to part 1
pub fn part_1(input: &str) -> String {
    let input_iter = input.split("\n");

    let count = input_iter
        .filter_map(|line| {
            let (left, right) = line.split_once(',')?;

            let (l1, l2) = left.split_once('-')?;
            let (r1, r2) = right.split_once('-')?;

            let (l1, l2): (u32, u32) = (l1.parse().ok()?, l2.parse().ok()?);
            let (r1, r2): (u32, u32) = (r1.parse().ok()?, r2.parse().ok()?);

            ((l1 <= r1 && r2 <= l2) || (r1 <= l1 && l2 <= r2)).then_some(())
        })
        .count();

    count.to_string()
}

/// Compute the solution to part 2
pub fn part_2(input: &str) -> String {
    let input_iter = input.split("\n");

    let count = input_iter
        .filter_map(|line| {
            let (left, right) = line.split_once(',')?;

            let (l1, l2) = left.split_once('-')?;
            let (r1, r2) = right.split_once('-')?;

            let (l1, l2): (u32, u32) = (l1.parse().ok()?, l2.parse().ok()?);
            let (r1, r2): (u32, u32) = (r1.parse().ok()?, r2.parse().ok()?);

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
