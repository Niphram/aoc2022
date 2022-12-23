use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self) -> [Self; 8] {
        [
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x + 1, self.y + 1),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x - 1, self.y - 1),
        ]
    }
}

fn print_map(map: &HashSet<Pos>) {
    let max_x = map.iter().map(|p| p.x).max().unwrap();
    let min_x = map.iter().map(|p| p.x).min().unwrap();
    let max_y = map.iter().map(|p| p.y).max().unwrap();
    let min_y = map.iter().map(|p| p.y).min().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&Pos::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Pos::new(x as isize, y as isize))
        })
        .collect::<HashSet<_>>();

    let checks = [
        [true, true, false, false, false, false, false, true],
        [false, false, false, true, true, true, false, false],
        [false, false, false, false, false, true, true, true],
        [false, true, true, true, false, false, false, false],
    ];

    for i in 0..10 {
        let proposed_positions = elves
            .iter()
            .map(|pos| {
                let Pos { x, y } = pos;

                let neighbors = pos.neighbors().map(|p| elves.get(&p).is_some());

                let new_pos = if neighbors.iter().all(|v| !v) {
                    *pos
                } else {
                    let moves = [
                        Pos::new(*x, y - 1),
                        Pos::new(*x, y + 1),
                        Pos::new(x - 1, *y),
                        Pos::new(x + 1, *y),
                    ];

                    let rules = checks
                        .map(|check| {
                            check
                                .iter()
                                .zip(neighbors)
                                .map(|(l, r)| *l && r)
                                .all(|v| !v)
                        })
                        .into_iter()
                        .zip(moves)
                        .collect::<Vec<_>>();

                    if let (true, new_pos) = rules[i % 4] {
                        new_pos
                    } else if let (true, new_pos) = rules[(i + 1) % 4] {
                        new_pos
                    } else if let (true, new_pos) = rules[(i + 2) % 4] {
                        new_pos
                    } else if let (true, new_pos) = rules[(i + 3) % 4] {
                        new_pos
                    } else {
                        *pos
                    }
                };

                (pos, new_pos)
            })
            .collect::<HashMap<_, _>>();

        elves = proposed_positions
            .iter()
            .map(|(&pos, new_pos)| {
                if proposed_positions
                    .values()
                    .filter(|&p| *p == *new_pos)
                    .count()
                    > 1
                {
                    *pos
                } else {
                    *new_pos
                }
            })
            .collect();
    }

    let max_x = elves.iter().map(|p| p.x).max().unwrap();
    let min_x = elves.iter().map(|p| p.x).min().unwrap();
    let max_y = elves.iter().map(|p| p.y).max().unwrap();
    let min_y = elves.iter().map(|p| p.y).min().unwrap();

    let area = (max_x.abs_diff(min_x) + 1) * (max_y.abs_diff(min_y) + 1);
    println!("--- ITERATION {} ---", 10);
    print_map(&elves);

    (area - elves.len()).to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Pos::new(x as isize, y as isize))
        })
        .collect::<HashSet<_>>();

    let checks = [
        [true, true, false, false, false, false, false, true],
        [false, false, false, true, true, true, false, false],
        [false, false, false, false, false, true, true, true],
        [false, true, true, true, false, false, false, false],
    ];

    for i in 0.. {
        let proposed_positions = elves
            .iter()
            .map(|pos| {
                let Pos { x, y } = pos;

                let neighbors = pos.neighbors().map(|p| elves.get(&p).is_some());

                let new_pos = if neighbors.iter().all(|v| !v) {
                    *pos
                } else {
                    let moves = [
                        Pos::new(*x, y - 1),
                        Pos::new(*x, y + 1),
                        Pos::new(x - 1, *y),
                        Pos::new(x + 1, *y),
                    ];

                    let rules = checks
                        .map(|check| {
                            check
                                .iter()
                                .zip(neighbors)
                                .map(|(l, r)| *l && r)
                                .all(|v| !v)
                        })
                        .into_iter()
                        .zip(moves)
                        .collect::<Vec<_>>();

                    if let (true, new_pos) = rules[i % 4] {
                        new_pos
                    } else if let (true, new_pos) = rules[(i + 1) % 4] {
                        new_pos
                    } else if let (true, new_pos) = rules[(i + 2) % 4] {
                        new_pos
                    } else if let (true, new_pos) = rules[(i + 3) % 4] {
                        new_pos
                    } else {
                        *pos
                    }
                };

                (pos, new_pos)
            })
            .collect::<HashMap<_, _>>();

        if proposed_positions
            .iter()
            .all(|(&pos, new_pos)| pos == new_pos)
        {
            return (i + 1).to_string();
        }

        elves = proposed_positions
            .iter()
            .map(|(&pos, new_pos)| {
                if proposed_positions
                    .values()
                    .filter(|&p| *p == *new_pos)
                    .count()
                    > 1
                {
                    *pos
                } else {
                    *new_pos
                }
            })
            .collect();
    }

    unreachable!()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 23");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
