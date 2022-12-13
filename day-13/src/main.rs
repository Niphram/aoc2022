use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Values(Vec<usize>),
    List(Vec<Packet>),
}

/// Implement ordering for our packet-struct
/// Makes heavy use of rusts default ordering of Vec's (lexicographical comparison)
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // If both sides are Packet::Values, just compare the Vec<usize>
            (Self::Values(left), Self::Values(right)) => left.cmp(right),
            // If both sides are Packet::List, compare the underlying Vec<Packet>
            (Self::List(left), Self::List(right)) => left.cmp(right),
            // If the sides aren't the same, upgrade one side to a Vec<Packet> and compare Vec's
            (Self::List(left), right) => left.cmp(&vec![right.clone()]),
            (left, Self::List(right)) => vec![left.clone()].cmp(right),
        }
    }
}

/// Needs to be implemented manually, if Ord is implemented
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(input: &str) -> Packet {
    // Try parsing comma seperated integers
    let list = input
        .split(',')
        .map(&str::parse::<usize>)
        .collect::<Vec<_>>();

    if list.iter().all(Result::is_ok) {
        // If parsing was successful, return Packet::Values
        Packet::Values(list.into_iter().map(|v| v.unwrap()).collect())
    } else {
        // Split into smaller lists

        // Function to find the next part
        fn next_part(input: &str) -> Option<(&str, &str)> {
            // Return none if input is empty
            if input.is_empty() {
                return None;
            }

            let end = input
                .chars()
                // Parse brackets to find depth of each character
                .scan(0, |depth, c| {
                    match c {
                        '[' => *depth += 1,
                        ']' => *depth -= 1,
                        _ => {}
                    };

                    Some((*depth, c))
                })
                // Find the next '|' that is at depth 0
                .position(|(depth, c)| depth == 0 && c == '|');

            if let Some(end) = end {
                // Return both parts if found
                Some((&input[..end], &input[end + 1..]))
            } else {
                // Otherwise return rest of string and empty slice
                Some((input, &input[0..0]))
            }
        }

        // Collect all Packets
        let mut output = vec![];

        // Repeat until no packets remain
        let mut next = &input[1..input.len() - 1];
        while let Some((part, rest)) = next_part(next) {
            output.push(parse_line(part));

            next = rest;
        }

        Packet::List(output)
    }
}

fn parse_input(input: &str) -> Vec<Packet> {
    // Use different symbols to seperate lists from values
    let input = input.replace(",[", "|[").replace("],", "]|");

    // Parse all packets, ignore empty lines
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let packets = parse_input(input);

    let res: usize = packets
        .chunks(2)
        // Enumerate to get indices
        .enumerate()
        // Filter only packets that are correctly ordered
        .filter_map(|(idx, packets)| (packets[0] < packets[1]).then_some(idx + 1))
        .sum();

    res.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut parsed = parse_input(input);

    // Create marker packets
    let pack_a = parse_line("[[2]]");
    let pack_b = parse_line("[[6]]");

    // Insert markers into packets
    parsed.push(pack_a.clone());
    parsed.push(pack_b.clone());

    // Sort packets
    parsed.sort();

    // Find markers in sorted packets
    let idx_a = parsed.iter().position(|p| p == &pack_a).unwrap() + 1;
    let idx_b = parsed.iter().position(|p| p == &pack_b).unwrap() + 1;

    (idx_a * idx_b).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 13");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
