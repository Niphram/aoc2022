#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
    Executing,
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let mut instructions = input.lines().flat_map(|l| match &l[0..4] {
        "noop" => vec![Instruction::Noop],
        "addx" => vec![
            Instruction::Executing,
            Instruction::Addx(l[5..].parse().unwrap()),
        ],
        _ => panic!("Unknown instruction"),
    });

    let mut x_reg = 1;
    let mut cycle = 1;

    let mut signal_strength = 0;
    while let Some(ins) = instructions.next() {
        if (cycle + 20) % 40 == 0 {
            signal_strength += cycle * x_reg;
        }

        if let Instruction::Addx(v) = ins {
            x_reg += v;
        }

        cycle += 1;
    }

    signal_strength.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut instructions = input.lines().flat_map(|l| match &l[0..4] {
        "noop" => vec![Instruction::Noop],
        "addx" => vec![
            Instruction::Executing,
            Instruction::Addx(l[5..].parse().unwrap()),
        ],
        _ => panic!("Unknown instruction"),
    });

    let mut x_reg = 1;
    let mut cycle = 0;

    while let Some(ins) = instructions.next() {
        let sprite = (x_reg - 1)..=(x_reg + 1);

        if sprite.contains(&(cycle % 40)) {
            print!("{}", "#");
        } else {
            print!("{}", ".");
        }

        cycle += 1;

        if cycle % 40 == 0 {
            println!();
        }

        if let Instruction::Addx(v) = ins {
            x_reg += v;
        }
    }

    String::new()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 10");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
