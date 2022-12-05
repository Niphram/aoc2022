/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    // Split input into lines
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks_str = stacks.split("\n").collect::<Vec<_>>();
    stacks_str.reverse();

    let mut stacks = vec![];

    for idx in (0..=stacks_str[0].len()).step_by(4) {
        let items = stacks_str[1..]
            .iter()
            .map_while(|line| {
                let item = &line[idx + 1..idx + 2];

                (item != " ").then_some(item)
            })
            .collect::<Vec<_>>();

        stacks.push(items);
    }

    for instruction in instructions.split("\n") {
        let mut parser = instruction
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|n| n.parse::<usize>().unwrap());

        let count = parser.next().unwrap();
        let from = parser.next().unwrap() - 1;
        let to = parser.next().unwrap() - 1;

        for _ in 0..count {
            let item = stacks.get_mut(from).unwrap().pop().unwrap();
            stacks.get_mut(to).unwrap().push(item);
        }
    }

    stacks
        .iter()
        .map(|s| *s.last().unwrap())
        .collect::<Vec<&str>>()
        .join("")
        .into()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    // Split input into lines
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks_str = stacks.split("\n").collect::<Vec<_>>();
    stacks_str.reverse();

    let mut stacks = vec![];

    for idx in (0..=stacks_str[0].len()).step_by(4) {
        let items = stacks_str[1..]
            .iter()
            .map_while(|line| {
                let item = &line[idx + 1..idx + 2];

                (item != " ").then_some(item)
            })
            .collect::<Vec<_>>();

        stacks.push(items);
    }

    for instruction in instructions.split("\n") {
        let mut parser = instruction
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|n| n.parse::<usize>().unwrap());

        let count = parser.next().unwrap();
        let from = parser.next().unwrap() - 1;
        let to = parser.next().unwrap() - 1;

        let mut temp = vec![];

        for _ in 0..count {
            temp.push(stacks.get_mut(from).unwrap().pop().unwrap());
        }

        temp.reverse();

        stacks.get_mut(to).unwrap().append(&mut temp);
    }

    stacks
        .iter()
        .map(|s| *s.last().unwrap())
        .collect::<Vec<&str>>()
        .join("")
        .into()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 5");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
