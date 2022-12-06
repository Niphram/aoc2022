/// Find the first substring of `length` characters and returns it's index
fn find_marker(input: &str, length: usize) -> Option<usize> {
    // This will only work with an ascii input
    if !input.is_ascii() {
        return None;
    }

    let idx = input
        // Treat input as a byte slice
        .as_bytes()
        // Look at `length` bytes at a time
        .windows(length)
        // Find the first window, where every byte is unique
        .position(|w| w.iter().collect::<std::collections::HashSet<_>>().len() == length);

    // Add `length` to correct the index
    idx.and_then(|idx| Some(idx + length))
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 6");

    let part_1 = find_marker(&input, 4).expect("Index of marker");
    let part_2 = find_marker(&input, 14).expect("Index of marker");

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
