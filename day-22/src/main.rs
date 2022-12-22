use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
    Nothing,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Nothing,
            '.' => Self::Floor,
            '#' => Self::Wall,
            _ => panic!(),
        }
    }
}

struct Board {
    grid: Vec<Vec<Tile>>,
}

impl Board {
    fn find_start(&self) -> (usize, usize) {
        let x = self.grid[0]
            .iter()
            .position(|t| *t != Tile::Nothing)
            .unwrap();

        (x, 0)
    }

    fn walk_direction(&self, (x, y): (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let (mut x, mut y) = (x as isize, y as isize);

        let (x_delta, y_delta) = match dir {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        };

        let (mut new_x, mut new_y) = (x + x_delta, y + y_delta);

        if new_x < 0
            || new_y < 0
            || self
                .grid
                .get(new_y as usize)
                .and_then(|r| r.get(new_x as usize))
                .and_then(|t| (*t != Tile::Nothing).then_some(()))
                .is_none()
        {
            // WRAP
            (new_x, new_y) = (x - x_delta, y - y_delta);

            while new_x >= 0
                && new_y >= 0
                && self
                    .grid
                    .get(new_y as usize)
                    .and_then(|r| r.get(new_x as usize))
                    .and_then(|t| (*t != Tile::Nothing).then_some(()))
                    .is_some()
            {
                (new_x, new_y) = (new_x - x_delta, new_y - y_delta);
            }

            (x, y) = (new_x, new_y);
        }

        let (new_x, new_y) = ((x + x_delta) as usize, (y + y_delta) as usize);

        match self.grid[new_y][new_x] {
            Tile::Floor => Some((new_x, new_y)),
            _ => None,
        }
    }
}

enum Instruction {
    Walk(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

type CubeFace = Vec<Vec<Tile>>;

struct Cube<const SIZE: usize> {
    front: CubeFace,
    top: CubeFace,
    bottom: CubeFace,
    left: CubeFace,
    right: CubeFace,
    back: CubeFace,
}

impl<const SIZE: usize> FromStr for Cube<SIZE> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let faces = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>());

        todo!()
    }
}

fn parse_puzzle_input(input: &str) -> (Board, Vec<Instruction>) {
    let (grid, instructions) = input.split_once("\n\n").unwrap();

    let grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    ' ' => Tile::Nothing,
                    '.' => Tile::Floor,
                    '#' => Tile::Wall,
                    _ => panic!("Unknown tile!"),
                })
                .collect()
        })
        .collect();

    let instructions = instructions
        .replace('R', " R ")
        .replace('L', " L ")
        .split(' ')
        .map(|i| match i {
            "L" => Instruction::TurnLeft,
            "R" => Instruction::TurnRight,
            w => Instruction::Walk(w.parse().unwrap()),
        })
        .collect();

    (Board { grid }, instructions)
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let (grid, instructions) = parse_puzzle_input(input);

    let mut pos = (grid.find_start(), Direction::E);

    for instruction in &instructions {
        match instruction {
            Instruction::TurnLeft => pos.1.turn_left(),
            Instruction::TurnRight => pos.1.turn_right(),
            Instruction::Walk(d) => {
                for _ in 0..*d {
                    if let Some(new_pos) = grid.walk_direction(pos.0, pos.1) {
                        pos.0 = new_pos;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let score = (pos.0 .1 + 1) * 1000
        + (pos.0 .0 + 1) * 4
        + match pos.1 {
            Direction::E => 0,
            Direction::S => 1,
            Direction::W => 2,
            Direction::N => 3,
        };

    score.to_string()
}

fn chunk(a: isize, chunks_size: isize) -> isize {
    if a >= 0 {
        a / chunks_size
    } else {
        -1
    }
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    const S: isize = 50;

    let mut edges = HashMap::new();

    edges.insert((1, 0, Direction::N), (0, 3, Direction::E));
    edges.insert((1, 0, Direction::E), (2, 0, Direction::E));
    edges.insert((1, 0, Direction::S), (1, 1, Direction::S));
    edges.insert((1, 0, Direction::W), (0, 2, Direction::E));

    edges.insert((2, 0, Direction::N), (0, 3, Direction::N));
    edges.insert((2, 0, Direction::E), (1, 2, Direction::W));
    edges.insert((2, 0, Direction::S), (1, 1, Direction::W));
    edges.insert((2, 0, Direction::W), (1, 0, Direction::W));

    edges.insert((1, 1, Direction::N), (1, 0, Direction::N));
    edges.insert((1, 1, Direction::E), (2, 0, Direction::N));
    edges.insert((1, 1, Direction::S), (1, 2, Direction::S));
    edges.insert((1, 1, Direction::W), (0, 2, Direction::S));

    edges.insert((0, 2, Direction::N), (1, 1, Direction::E));
    edges.insert((0, 2, Direction::E), (1, 2, Direction::E));
    edges.insert((0, 2, Direction::S), (0, 3, Direction::S));
    edges.insert((0, 2, Direction::W), (1, 0, Direction::E));

    edges.insert((1, 2, Direction::N), (1, 1, Direction::N));
    edges.insert((1, 2, Direction::E), (2, 0, Direction::W));
    edges.insert((1, 2, Direction::S), (0, 3, Direction::W));
    edges.insert((1, 2, Direction::W), (0, 2, Direction::W));

    edges.insert((0, 3, Direction::N), (0, 2, Direction::N));
    edges.insert((0, 3, Direction::E), (1, 2, Direction::N));
    edges.insert((0, 3, Direction::S), (2, 0, Direction::S));
    edges.insert((0, 3, Direction::W), (1, 0, Direction::S));

    let (grid, instructions) = parse_puzzle_input(input);

    let (pos, mut dir) = (grid.find_start(), Direction::E);
    let mut pos = (pos.0 as isize, pos.1 as isize);

    for instruction in &instructions {
        match instruction {
            Instruction::TurnLeft => dir.turn_left(),
            Instruction::TurnRight => dir.turn_right(),
            Instruction::Walk(d) => {
                for _ in 0..*d {
                    let current_face = (chunk(pos.0, S), chunk(pos.1, S));

                    let mut new_pos = match dir {
                        Direction::N => (pos.0, pos.1 - 1),
                        Direction::E => (pos.0 + 1, pos.1),
                        Direction::S => (pos.0, pos.1 + 1),
                        Direction::W => (pos.0 - 1, pos.1),
                    };

                    let mut new_dir = dir;

                    if current_face != (chunk(new_pos.0, S), chunk(new_pos.1, S)) {
                        let new_face = edges[&(current_face.0, current_face.1, dir)];

                        let pos_on_new_face = (new_pos.0.rem_euclid(S), new_pos.1.rem_euclid(S));

                        let pos_on_new_face = match (dir, new_face.2) {
                            (a, b) if a == b => pos_on_new_face,
                            (Direction::N, Direction::E) => (0, pos_on_new_face.0),
                            (Direction::E, Direction::N) => (pos_on_new_face.1, S - 1),
                            (Direction::E, Direction::W) => (S - 1, S - 1 - pos_on_new_face.1),
                            (Direction::S, Direction::W) => (S - 1, pos_on_new_face.0),
                            (Direction::W, Direction::E) => (0, S - 1 - pos_on_new_face.1),
                            (Direction::W, Direction::S) => (pos_on_new_face.1, 0),
                            _ => panic!(),
                        };

                        new_pos = (
                            new_face.0 * S + pos_on_new_face.0,
                            new_face.1 * S + pos_on_new_face.1,
                        );

                        new_dir = new_face.2;
                    }

                    if grid.grid[new_pos.1 as usize][new_pos.0 as usize] == Tile::Floor {
                        pos = new_pos;
                        dir = new_dir;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let score = (pos.1 + 1) * 1000
        + (pos.0 + 1) * 4
        + match dir {
            Direction::E => 0,
            Direction::S => 1,
            Direction::W => 2,
            Direction::N => 3,
        };

    score.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 22");

    dbg!(4i32.rem_euclid(4));

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
