#[derive(Debug)]
enum WorryFunction {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct PassData {
    test: u64,
    yes: usize,
    no: usize,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    worry_function: WorryFunction,
    pass_data: PassData,
    inspected_items: usize,
}

impl Monkey {
    /// Inspects all items and applies worry function
    fn inspect_items(&mut self) {
        self.items
            .iter_mut()
            .for_each(|i| match self.worry_function {
                WorryFunction::Add(v) => *i += v,
                WorryFunction::Multiply(v) => *i *= v,
                WorryFunction::Square => *i *= *i,
            });

        self.inspected_items += self.items.len();
    }

    /// Drains the items and returns a vector of item and the monkey-id they should go to
    fn pass_items(&mut self) -> Vec<(usize, u64)> {
        self.items
            .drain(..)
            .map(|i| match i % self.pass_data.test {
                0 => (self.pass_data.yes, i),
                _ => (self.pass_data.no, i),
            })
            .collect()
    }
}

/// Parse input to monkey
fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<_> = input.lines().collect();

    // Get all items
    let items: Vec<u64> = lines[1][18..].split(", ").flat_map(&str::parse).collect();

    // Get worry function
    let worry_change = {
        let worry_number = lines[2][25..].parse();

        if let Ok(worry_number) = worry_number {
            match &lines[2][23..24] {
                "*" => WorryFunction::Multiply(worry_number),
                "+" => WorryFunction::Add(worry_number),
                _ => panic!("Unknown operation!"),
            }
        } else {
            WorryFunction::Square
        }
    };

    // Get pass-data
    let pass_data = {
        let test_number: u64 = lines[3][21..].parse().unwrap();
        let test_true: usize = lines[4][29..].parse().unwrap();
        let test_false: usize = lines[5][30..].parse().unwrap();

        PassData {
            test: test_number,
            yes: test_true,
            no: test_false,
        }
    };

    Monkey {
        items,
        worry_function: worry_change,
        pass_data,
        inspected_items: 0,
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let mut monkeys = input.split("\n\n").map(parse_monkey).collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            // Inspect all items
            monkey.inspect_items();

            // Divide worry-values by 3 (and implicitly floor)
            monkey.items.iter_mut().for_each(|i| {
                *i /= 3;
            });

            // Pass out items to other monkeys
            for item in monkey.pass_items() {
                monkeys[item.0].items.push(item.1);
            }
        }
    }

    // Sort by number of inspected items (highest first)
    monkeys.sort_unstable_by_key(|m| std::cmp::Reverse(m.inspected_items));

    (monkeys[0].inspected_items * monkeys[1].inspected_items).to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut monkeys = input.split("\n\n").map(parse_monkey).collect::<Vec<_>>();

    // Find the smallest common multiple. This works, because the divisible-by-checks are primes
    let lcm: u64 = monkeys.iter().map(|m| m.pass_data.test).product();

    for _ in 0..100000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            // Inspect all items
            monkey.inspect_items();

            // Use the calculated lcm to keep values in a commutative ring
            // Otherwise numbers would get too large
            monkey.items.iter_mut().for_each(|i| {
                *i %= lcm;
            });

            // Pass out items to other monkeys
            for item in monkey.pass_items() {
                monkeys[item.0].items.push(item.1);
            }
        }
    }

    // Sort by number of inspected items (highest first)
    monkeys.sort_unstable_by_key(|m| std::cmp::Reverse(m.inspected_items));

    (monkeys[0].inspected_items * monkeys[1].inspected_items).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 11");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
