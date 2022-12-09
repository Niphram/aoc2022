use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct RopeEnd {
    x: i32,
    y: i32,
}

impl RopeEnd {
    fn dis(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    fn move_to(&mut self, other: &Self) {
        if self.dis(other) > 1 {
            let x_diff = other.x - self.x;
            let y_diff = other.y - self.y;

            self.x += x_diff.signum();
            self.y += y_diff.signum();
        }
    }

    fn move_dir(&mut self, dir: Move) {
        match dir {
            Move::Up => self.y -= 1,
            Move::Down => self.y += 1,
            Move::Left => self.x -= 1,
            Move::Right => self.x += 1,
        }
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let moves = input.lines().flat_map(|l| {
        let (dir, count) = l.split_once(' ').unwrap();

        [match dir {
            "U" => Move::Up,
            "D" => Move::Down,
            "L" => Move::Left,
            "R" => Move::Right,
            _ => panic!("Unknown direction"),
        }]
        .repeat(count.parse::<usize>().unwrap())
    });

    let mut head = RopeEnd::default();
    let mut tail = RopeEnd::default();

    let mut visited = HashSet::from([RopeEnd::default()]);

    for m in moves {
        head.move_dir(m);
        tail.move_to(&head);

        visited.insert(tail);
    }

    visited.len().to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let moves = input.lines().flat_map(|l| {
        let (dir, count) = l.split_once(' ').unwrap();

        [match dir {
            "U" => Move::Up,
            "D" => Move::Down,
            "L" => Move::Left,
            "R" => Move::Right,
            _ => panic!("Unknown direction"),
        }]
        .repeat(count.parse::<usize>().unwrap())
    });

    let mut rope = [RopeEnd::default(); 10];

    let mut visited = HashSet::from([RopeEnd::default()]);

    for m in moves {
        rope[0].move_dir(m);

        for i in 1..rope.len() {
            let target = rope[i - 1];
            rope[i].move_to(&target);
        }

        visited.insert(*rope.last().unwrap());
    }

    visited.len().to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 9");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
