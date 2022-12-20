use std::{cmp::Ordering, ops::AddAssign, time::Instant};

struct Ring {
    data: Vec<isize>,
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let mut numbers = input
        .lines()
        .map(&str::parse::<isize>)
        .map(Result::unwrap)
        .enumerate()
        .collect::<Vec<_>>();

    let len = numbers.len();

    for i in 0..len {
        let mut pos = numbers.iter().position(|(idx, _)| *idx == i).unwrap();
        let val = numbers[pos];

        match val.1.cmp(&0) {
            Ordering::Greater => {
                for _ in 0..val.1 {
                    let from = pos % len;
                    let to = (from + 1) % len;

                    numbers.swap(from, to);

                    pos = to;
                }
            }
            Ordering::Less => {
                for _ in 0..val.1.abs() {
                    let from = pos % len;
                    let to = (from + len - 1) % len;

                    numbers.swap(from, to);

                    pos = to;
                }
            }
            _ => {}
        }
    }

    let zero_idx = numbers.iter().position(|(_, v)| *v == 0).unwrap();

    let a = (zero_idx + 1000) % len;
    let b = (zero_idx + 2000) % len;
    let c = (zero_idx + 3000) % len;

    (numbers[a].1 + numbers[b].1 + numbers[c].1).to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut numbers_orig = input
        .lines()
        .map(&str::parse::<isize>)
        .map(Result::unwrap)
        .map(|v| v * 811589153)
        .enumerate()
        .collect::<Vec<_>>();

    let len = numbers_orig.len();

    let mut numbers = numbers_orig
        .iter()
        .map(|(idx, n)| match n.cmp(&0) {
            Ordering::Greater => (*idx, *n % (len - 1) as isize),
            Ordering::Less => (*idx, n.rem_euclid(len as isize - 1)),
            Ordering::Equal => (*idx, *n),
        })
        .collect::<Vec<_>>();

    for _ in 0..10 {
        for i in 0..len {
            let mut pos = numbers.iter().position(|(idx, _)| *idx == i).unwrap();
            let val = numbers[pos];

            match val.1.cmp(&0) {
                Ordering::Greater => {
                    for _ in 0..val.1 {
                        let from = pos % len;
                        let to = (from + 1) % len;

                        numbers.swap(from, to);

                        pos = to;
                    }
                }
                Ordering::Less => {
                    for _ in 0..val.1.abs() {
                        let from = pos % len;
                        let to = (from + len - 1) % len;

                        numbers.swap(from, to);

                        pos = to;
                    }
                }
                _ => {}
            }
        }

        println!("ITERATE!");
    }

    let zero_idx = numbers.iter().position(|(_, v)| *v == 0).unwrap();

    let (idx_a, _) = numbers[(zero_idx + 1000) % len];
    let (idx_b, _) = numbers[(zero_idx + 2000) % len];
    let (idx_c, _) = numbers[(zero_idx + 3000) % len];

    let num_a = numbers_orig.iter().find(|(i, n)| *i == idx_a).unwrap();
    let num_b = numbers_orig.iter().find(|(i, n)| *i == idx_b).unwrap();
    let num_c = numbers_orig.iter().find(|(i, n)| *i == idx_c).unwrap();

    (num_a.1 + num_b.1 + num_c.1).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 20");

    let start = Instant::now();
    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}. Took {:?}", start.elapsed());

    let start = Instant::now();
    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}. Took {:?}", start.elapsed());
}
