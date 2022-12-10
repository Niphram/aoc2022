#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

/// Returns the value of the x-register after every cycle
/// Starts with cycle 0 ('during first cycle')
fn execute_input(input: &str) -> Vec<isize> {
    let mut x_reg = 1;

    // Pad the start to include cycle 0
    std::iter::once(x_reg)
        .chain(
            input
                .lines()
                .flat_map(|l| match &l[0..4] {
                    "noop" => vec![Instruction::Noop],
                    "addx" => vec![
                        // Pad every Addx with a Noop to make timing easy
                        Instruction::Noop,
                        Instruction::Addx(l[5..].parse().unwrap()),
                    ],
                    _ => panic!("Unknown instruction"),
                })
                .map(|ins| {
                    // Execute all instructions and return x-register
                    if let Instruction::Addx(v) = ins {
                        x_reg += v;
                    }

                    x_reg
                }),
        )
        .collect()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let register_trace = execute_input(input);

    let signal_strength: isize = register_trace
        .iter()
        .enumerate()
        // Only include cycle 20, 60, 100 and so on
        .filter(|(cycle, _)| (cycle + 21) % 40 == 0)
        // Calculate signal strength
        .map(|(cycle, x_reg)| (cycle as isize + 1) * x_reg)
        .sum();

    signal_strength.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let register_trace = execute_input(input);

    // Ignore last element
    register_trace[0..register_trace.len() - 1]
        // Chunk into lines
        .chunks(40)
        .map(|line| {
            // Create output line
            line.iter()
                .enumerate()
                // Select correct pixel
                .map(|(pixel, x_reg)| match pixel as isize {
                    c @ _ if c >= x_reg - 1 && c <= x_reg + 1 => '▮',
                    _ => ' ',
                })
                .collect::<String>()
        })
        // Collect into multiline string
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 10");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2:");
    println!("{part_2}");
}
