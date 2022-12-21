use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Numbers<'a> {
    Val(isize),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

impl<'a> From<&'a str> for Numbers<'a> {
    fn from(s: &'a str) -> Self {
        if let Ok(val) = s.parse::<isize>() {
            Numbers::Val(val)
        } else {
            match s.chars().nth(5) {
                Some('+') => Numbers::Add(&s[..4], &s[7..]),
                Some('-') => Numbers::Sub(&s[..4], &s[7..]),
                Some('*') => Numbers::Mul(&s[..4], &s[7..]),
                Some('/') => Numbers::Div(&s[..4], &s[7..]),
                _ => panic!(),
            }
        }
    }
}

fn get_monkey_value(name: &str, monkeys: &HashMap<&str, Numbers>) -> isize {
    let monkey = &monkeys[name];

    match monkey {
        Numbers::Val(val) => *val,
        Numbers::Add(a, b) => get_monkey_value(a, monkeys) + get_monkey_value(b, monkeys),
        Numbers::Sub(a, b) => get_monkey_value(a, monkeys) - get_monkey_value(b, monkeys),
        Numbers::Mul(a, b) => get_monkey_value(a, monkeys) * get_monkey_value(b, monkeys),
        Numbers::Div(a, b) => get_monkey_value(a, monkeys) / get_monkey_value(b, monkeys),
    }
}

fn contains_humn(name: &str, monkeys: &HashMap<&str, Numbers>) -> bool {
    if name == "humn" {
        true
    } else {
        let monkey = &monkeys[name];

        match monkey {
            Numbers::Val(val) => false,
            Numbers::Add(a, b) => contains_humn(a, monkeys) || contains_humn(b, monkeys),
            Numbers::Sub(a, b) => contains_humn(a, monkeys) || contains_humn(b, monkeys),
            Numbers::Mul(a, b) => contains_humn(a, monkeys) || contains_humn(b, monkeys),
            Numbers::Div(a, b) => contains_humn(a, monkeys) || contains_humn(b, monkeys),
        }
    }
}

fn solve_for_humn(name: &str, target: isize, monkeys: &HashMap<&str, Numbers>) -> isize {
    if name == "humn" {
        target
    } else {
        let monkey = &monkeys[name];

        match monkey {
            Numbers::Add(a, b) => {
                if contains_humn(a, monkeys) {
                    solve_for_humn(a, target - get_monkey_value(b, monkeys), monkeys)
                } else if contains_humn(b, monkeys) {
                    solve_for_humn(b, target - get_monkey_value(a, monkeys), monkeys)
                } else {
                    panic!()
                }
            }
            Numbers::Sub(a, b) => {
                if contains_humn(a, monkeys) {
                    solve_for_humn(a, get_monkey_value(b, monkeys) + target, monkeys)
                } else if contains_humn(b, monkeys) {
                    solve_for_humn(b, get_monkey_value(a, monkeys) - target, monkeys)
                } else {
                    panic!()
                }
            }
            Numbers::Mul(a, b) => {
                if contains_humn(a, monkeys) {
                    solve_for_humn(a, target / get_monkey_value(b, monkeys), monkeys)
                } else if contains_humn(b, monkeys) {
                    solve_for_humn(b, target / get_monkey_value(a, monkeys), monkeys)
                } else {
                    panic!()
                }
            }
            Numbers::Div(a, b) => {
                if contains_humn(a, monkeys) {
                    solve_for_humn(a, get_monkey_value(b, monkeys) * target, monkeys)
                } else if contains_humn(b, monkeys) {
                    solve_for_humn(b, get_monkey_value(a, monkeys) / target, monkeys)
                } else {
                    panic!()
                }
            }
            _ => panic!("Should not get a value answer"),
        }
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let monkeys = input
        .lines()
        .map(|line| {
            let (monkey, answer) = line.split_once(": ").unwrap();
            let answer: Numbers = answer.into();
            (monkey, answer)
        })
        .collect::<HashMap<_, _>>();

    get_monkey_value("root", &monkeys).to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut monkeys = input
        .lines()
        .map(|line| {
            let (monkey, answer) = line.split_once(": ").unwrap();
            let answer: Numbers = answer.into();
            (monkey, answer)
        })
        .collect::<HashMap<_, _>>();

    let root = monkeys.remove("root").unwrap();

    let (left, right) = match root {
        Numbers::Add(a, b) => (a, b),
        Numbers::Sub(a, b) => (a, b),
        Numbers::Mul(a, b) => (a, b),
        Numbers::Div(a, b) => (a, b),
        _ => panic!(),
    };

    let (humn_branch, target) = if contains_humn(left, &monkeys) {
        // Human is in the left branch
        (left, get_monkey_value(right, &monkeys))
    } else if contains_humn(right, &monkeys) {
        // Human is in the right branch
        (right, get_monkey_value(left, &monkeys))
    } else {
        panic!("Human is unused?")
    };

    solve_for_humn(humn_branch, target, &monkeys).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 21");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
