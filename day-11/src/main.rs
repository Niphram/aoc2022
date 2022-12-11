#[derive(Debug)]
enum WorryChanger {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct PassData {
    divisible_by: u64,
    yes: usize,
    no: usize,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    worry_change: WorryChanger,
    pass_data: PassData,
    inspected_items: usize,
}

impl Monkey {
    fn inspect_items(&mut self) {
        self.items.iter_mut().for_each(|i| match self.worry_change {
            WorryChanger::Add(v) => *i += v,
            WorryChanger::Multiply(v) => *i *= v,
            WorryChanger::Square => *i *= *i,
        });

        self.inspected_items += self.items.len();
    }

    fn pass_items(&mut self) -> Vec<(usize, u64)> {
        self.items
            .drain(..)
            .map(|i| match i % self.pass_data.divisible_by {
                0 => (self.pass_data.yes, i),
                _ => (self.pass_data.no, i),
            })
            .collect()
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<_> = input.lines().collect();

    let items: Vec<u64> = lines[1][18..].split(", ").flat_map(&str::parse).collect();

    let worry_change = {
        let worry_number = lines[2][25..].parse();

        if let Ok(worry_number) = worry_number {
            match &lines[2][23..24] {
                "*" => WorryChanger::Multiply(worry_number),
                "+" => WorryChanger::Add(worry_number),
                _ => panic!("Unknown operation!"),
            }
        } else {
            WorryChanger::Square
        }
    };

    let pass_data = {
        let test_number: u64 = lines[3][21..].parse().unwrap();
        let test_true: usize = lines[4][29..].parse().unwrap();
        let test_false: usize = lines[5][30..].parse().unwrap();

        PassData {
            divisible_by: test_number,
            yes: test_true,
            no: test_false,
        }
    };

    Monkey {
        items,
        worry_change,
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
            monkey.inspect_items();

            monkey.items.iter_mut().for_each(|i| {
                *i /= 3;
            });

            for item in monkey.pass_items() {
                monkeys[item.0].items.push(item.1);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| m.inspected_items);
    monkeys.reverse();
    (monkeys[0].inspected_items * monkeys[1].inspected_items).to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut monkeys = input.split("\n\n").map(parse_monkey).collect::<Vec<_>>();

    let lcm: u64 = monkeys.iter().map(|m| m.pass_data.divisible_by).product();

    for _ in 0..100000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            monkey.inspect_items();

            monkey.items.iter_mut().for_each(|i| {
                *i %= lcm;
            });

            for item in monkey.pass_items() {
                monkeys[item.0].items.push(item.1);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| m.inspected_items);
    monkeys.reverse();
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
