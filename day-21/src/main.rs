use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
}

#[derive(Debug)]
enum MonkeyYell<'a> {
    Value(usize),
    Equation(&'a str, &'a str, Operation),
}

impl<'a> From<&'a str> for MonkeyYell<'a> {
    fn from(s: &'a str) -> Self {
        if let Ok(val) = s.parse::<usize>() {
            Self::Value(val)
        } else {
            match s.chars().nth(5) {
                Some('+') => Self::Equation(&s[..4], &s[7..], Operation::Add),
                Some('-') => Self::Equation(&s[..4], &s[7..], Operation::Sub),
                Some('*') => Self::Equation(&s[..4], &s[7..], Operation::Mul),
                Some('/') => Self::Equation(&s[..4], &s[7..], Operation::Div),
                Some(op) => panic!("Unknown operation = '{op}'"),
                None => panic!("Could not read operation!"),
            }
        }
    }
}

/// Traverses the tree of monkeys and returns the resulting value
fn get_monkey_value(name: &str, monkeys: &HashMap<&str, MonkeyYell>) -> usize {
    match &monkeys[name] {
        // If monkey is shouting a number, return it
        MonkeyYell::Value(val) => *val,
        // Otherwise check both monkeys this monkey is listening for and compute answer
        MonkeyYell::Equation(a, b, op) => {
            let a = get_monkey_value(a, monkeys);
            let b = get_monkey_value(b, monkeys);

            match op {
                Operation::Add => a + b,
                Operation::Sub => a - b,
                Operation::Mul => a * b,
                Operation::Div => a / b,
                _ => panic!("Unsupported operation!"),
            }
        }
    }
}

/// Checks if the monkey includes the human in their answer
fn listens_to_human(name: &str, monkeys: &HashMap<&str, MonkeyYell>) -> bool {
    if name == "humn" {
        true
    } else {
        match &monkeys[name] {
            // A monkey just yelling a number does not listen to the human
            MonkeyYell::Value(_) => false,
            // Check both 'monkeys' that this monkey is listening to
            MonkeyYell::Equation(a, b, _op) => {
                listens_to_human(a, monkeys) || listens_to_human(b, monkeys)
            }
        }
    }
}

/// Given a monkey that has to wait for the human and a target
/// value, find the value the human has to yell
fn solve_for_human(name: &str, target: usize, monkeys: &HashMap<&str, MonkeyYell>) -> usize {
    // If we found the human, just return the target value
    if name == "humn" {
        target
    } else {
        match &monkeys[name] {
            MonkeyYell::Equation(a, b, op) => {
                // Check which side of the equation contains the human
                if listens_to_human(a, monkeys) {
                    // Find out the target-value for the left side
                    let target = match op {
                        Operation::Add => target - get_monkey_value(b, monkeys),
                        Operation::Mul => target / get_monkey_value(b, monkeys),
                        Operation::Sub => get_monkey_value(b, monkeys) + target,
                        Operation::Div => get_monkey_value(b, monkeys) * target,
                        Operation::Equal => get_monkey_value(b, monkeys),
                    };

                    solve_for_human(a, target, monkeys)
                } else if listens_to_human(b, monkeys) {
                    // Find out the target-value for the right side
                    let target = match op {
                        Operation::Add => target - get_monkey_value(a, monkeys),
                        Operation::Mul => target / get_monkey_value(a, monkeys),
                        Operation::Sub => get_monkey_value(a, monkeys) - target,
                        Operation::Div => get_monkey_value(a, monkeys) / target,
                        Operation::Equal => get_monkey_value(a, monkeys),
                    };

                    solve_for_human(b, target, monkeys)
                } else {
                    panic!("{name}'s answer is independent of the human!")
                }
            }
            MonkeyYell::Value(_) => panic!("{name}'s answer is independent of the human!"),
        }
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let monkeys = input
        .lines()
        .map(|line| {
            let (monkey, answer) = line.split_once(": ").unwrap();
            let answer: MonkeyYell = answer.into();
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
            let answer: MonkeyYell = answer.into();
            (monkey, answer)
        })
        .collect::<HashMap<_, _>>();

    // Change root monkey's operation to 'equal'
    if let MonkeyYell::Equation(a, b, _) = monkeys.remove("root").unwrap() {
        monkeys.insert("root", MonkeyYell::Equation(a, b, Operation::Equal));
    } else {
        panic!("Monkey 'root' can't yell a value!");
    }

    solve_for_human("root", 0, &monkeys).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 21");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
