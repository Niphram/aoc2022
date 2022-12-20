fn decrypt_coordinates<const ITERATIONS: usize>(numbers: &[isize]) -> isize {
    // Enumerate numbers to know their original indices
    let mut numbers = numbers.iter().enumerate().collect::<Vec<_>>();

    // Count numbers
    let len = numbers.len();

    // Repeat for specified iterations
    for _ in 0..ITERATIONS {
        // Go through all numbers in order
        for i in 0..len {
            // Find number in vec
            let idx = numbers
                .iter()
                .position(|(idx, _)| *idx == i)
                .expect("Find number by original index");

            // Remove from vec
            let (orig_idx, val) = numbers.remove(idx);

            // Find new index
            let new_idx = (idx as isize + val)
                .rem_euclid(len as isize - 1)
                .unsigned_abs();

            // Insert number at new position
            numbers.insert(new_idx, (orig_idx, val));
        }
    }

    // Find the index of the zero-value
    let zero_idx = numbers
        .iter()
        .position(|(_, &v)| v == 0)
        .expect("Find 0-value");

    // Sum values of the elements that are offset by 1000, 2000 and 3000 from the 0-Element
    [1000, 2000, 3000]
        .iter()
        .map(|o| (zero_idx + o) % len)
        .map(|idx| numbers[idx].1)
        .sum()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    // Parse numbers
    let numbers = input
        .lines()
        .flat_map(&str::parse::<isize>)
        .collect::<Vec<_>>();

    // Decrypt one iteration
    decrypt_coordinates::<1>(&numbers).to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    // Parse numbers and multiply by 811589153
    let numbers = input
        .lines()
        .flat_map(&str::parse::<isize>)
        .map(|v| v * 811589153)
        .collect::<Vec<_>>();

    // Decrypt ten iterations
    decrypt_coordinates::<10>(&numbers).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 20");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
