/// Mutably borrows two elements from a slice
fn index_mut_2<T>(slice: &mut [T], idx1: usize, idx2: usize) -> (&mut T, &mut T) {
    // Make sure we are not referencing the same elements
    assert!(idx1 != idx2);

    // Get raw pointers to elements
    let a: *mut T = &mut slice[idx1];
    let b: *mut T = &mut slice[idx2];

    // Dereference elements. Safety is guaranteed.
    unsafe { (&mut *a, &mut *b) }
}

/// Returns the initial state of the stacks
fn parse_initial_state(input: &str) -> Vec<Vec<char>> {
    let (input, header) = input.rsplit_once('\n').expect("Split off last line");

    // Find out how many stacks we need
    let stack_count = (header.len() + 1) / 4;

    let mut iters: Vec<_> = input
        // Reverse split and skip one line (The one with the stack numbers)
        .lines()
        .rev()
        .map(|line| {
            // Get all the crate labels
            line.chars()
                .skip(1)
                .step_by(4)
                .map(|c| (c != ' ').then_some(c))
        })
        .collect();

    // Build vector of stacks in the correct order
    (0..stack_count)
        .map(|_| {
            iters
                .iter_mut()
                // flat-map to remove None from the stacks
                .flat_map(|n| n.next().expect("Iterator can't be empty"))
                .collect()
        })
        .collect()
}

/// Parses an instruction like `move 1 from 2 to 3`
/// from and to reduced by one to make them index at 0
fn parse_instruction(line: &str) -> (usize, usize, usize) {
    let values = line
        .split(' ')
        // Skip one and step by two
        .skip(1)
        .step_by(2)
        .map(|n| n.parse().expect("Parse number in instruction"))
        .collect::<Vec<_>>();

    (values[0], values[1] - 1, values[2] - 1)
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    // Split input into lines
    let (stacks, instructions) = input.split_once("\n\n").expect("Split at empty line");

    // Parse state
    let mut stacks = parse_initial_state(stacks);

    // Execute all instructions
    for instruction in instructions.lines() {
        // Get parsed instructions
        let (count, from, to) = parse_instruction(instruction);

        let (from, to) = index_mut_2(&mut stacks, from, to);

        // Remove `count` elements from the end and reverse
        let mut moved = from.drain(from.len() - count..).rev();

        // Append moved items to target stack
        to.extend(&mut moved);
    }

    // Get top crates and return
    stacks
        .iter()
        .map(|s| s.last().expect("Stack should not be empty"))
        .collect()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    // Split input into lines
    let (stacks, instructions) = input.split_once("\n\n").expect("Split at empty line");

    let mut stacks = parse_initial_state(stacks);

    // Execute all instructions
    for instruction in instructions.lines() {
        // Get parsed instructions
        let (count, from, to) = parse_instruction(instruction);

        let (from_vec, to_vec) = index_mut_2(&mut stacks, from, to);

        // Remove `count` elements from the end
        let mut moved = from_vec.drain(from_vec.len() - count..);

        // Append moved items to target stack
        to_vec.extend(&mut moved);
    }

    // Get top crates and return
    stacks
        .iter()
        .map(|s| s.last().expect("Stack should not be empty"))
        .collect()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 5");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
