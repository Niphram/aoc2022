use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Nested {
    Values(Vec<usize>),
    List(Vec<Nested>),
}

impl Ord for Nested {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Values(left), Self::Values(right)) => left
                .iter()
                .zip(right)
                .find_map(|(left, right)| left.cmp(right).is_ne().then_some(left.cmp(right)))
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (Self::List(left), Self::List(right)) => left
                .iter()
                .zip(right)
                .find_map(|(left, right)| left.cmp(right).is_ne().then_some(left.cmp(right)))
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (Self::List(left), Self::Values(right)) => {
                Self::List(left.clone()).cmp(&Self::List(vec![Self::Values(right.clone())]))
            }
            (Self::Values(left), Self::List(right)) => {
                Self::List(vec![Self::Values(left.clone())]).cmp(&Self::List(right.clone()))
            }
        }
    }
}

impl PartialOrd for Nested {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(input: &str) -> Nested {
    let list = input
        .split(',')
        .map(&str::parse::<usize>)
        .collect::<Vec<_>>();

    if list.iter().all(Result::is_ok) {
        Nested::Values(list.into_iter().map(|v| v.unwrap()).collect())
    } else {
        let find_closing = |input: &str| {
            input
                .chars()
                .scan(0, |depth, c| {
                    match c {
                        '[' => {
                            *depth += 1;
                        }
                        ']' => {
                            *depth -= 1;
                        }
                        _ => {}
                    };

                    (*depth > 0).then_some(c)
                })
                .count()
        };

        let mut output = vec![];
        let mut rest = input;

        while !rest.is_empty() {
            match &rest[..1] {
                "[" => {
                    let end = find_closing(rest);
                    output.push(parse_line(&rest[1..end]));
                    rest = &rest[end + 1..];
                }
                "|" => {
                    if let Some(end) = rest.find('|') {
                        if end != 0 {
                            output.push(parse_line(&rest[1..end]));
                        }
                        rest = &rest[end + 1..];
                    } else {
                        output.push(parse_line(&rest[1..]));
                        break;
                    }
                }
                _ => {
                    if let Some(end) = rest.find(|c| c == '|' || c == ']') {
                        output.push(parse_line(&rest[..end]));
                        rest = &rest[end + 1..];
                    } else {
                        output.push(parse_line(rest));
                        break;
                    }
                }
            }
        }

        Nested::List(output)
    }
}

fn parse_input(input: &str) -> Vec<(Nested, Nested)> {
    let input = input.replace(",[", "|[").replace("],", "]|");
    let lines: Vec<_> = input.lines().collect();

    lines
        .chunks(3)
        .map(|chunk| (parse_line(chunk[0]), parse_line(chunk[1])))
        .collect()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let parsed = parse_input(input);

    let res: usize = parsed
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, ..)| i + 1)
        .sum();

    res.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let parsed = parse_input(input);

    let mut parsed: Vec<_> = parsed.iter().flat_map(|(l, r)| [l, r]).collect();

    let pack_a = parse_line("[[2]]");
    let pack_b = parse_line("[[6]]");

    parsed.push(&pack_a);
    parsed.push(&pack_b);

    parsed.sort();

    let idx_a = parsed.iter().position(|p| *p == &pack_a).unwrap() + 1;
    let idx_b = parsed.iter().position(|p| *p == &pack_b).unwrap() + 1;

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
