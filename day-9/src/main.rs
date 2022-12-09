use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Knot {
    x: i32,
    y: i32,
}

struct Rope<const N: usize> {
    knots: [Knot; N],
}

impl<const N: usize> Rope<N> {
    /// Creates a new rope
    fn new() -> Self {
        assert!(N >= 2, "A rope needs to be of length 2 or higher");

        Rope {
            knots: [Knot::default(); N],
        }
    }

    /// Moves the head of the rope in the specified direction and updates all other knots.
    /// Returns the last knot
    fn move_head(&mut self, dir: Direction) -> &Knot {
        // Move the head in the direction
        self.knots[0].move_dir(dir);

        self.knots
            .iter_mut()
            // Moves every knot in the direction of the preceeding knot
            .reduce(|current, next| next.move_to(current))
            //Never fails, because the rope is at least of length 2
            .unwrap()
    }
}

impl Knot {
    /// Returns the chebyshev distance between two knots
    fn dis(&self, other: &Self) -> u32 {
        u32::max(
            i32::abs_diff(self.x, other.x),
            i32::abs_diff(self.y, other.y),
        )
    }

    /// Moves the knot in the direction of another knot
    /// Only moves one step
    fn move_to(&mut self, other: &Self) -> &mut Self {
        // Only move, if the knots are too far apart
        if self.dis(other) > 1 {
            let x_diff = other.x - self.x;
            let y_diff = other.y - self.y;

            // This will only move the knot one step, but it correctly handles diagonals
            self.x += x_diff.signum();
            self.y += y_diff.signum();
        }

        self
    }

    /// Moves the knot in the given direction
    fn move_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

/// Parses the input to a Vec<Directions>
/// Multiple moves in the same direction will be flattened
fn parse_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .flat_map(|l| {
            let (dir, count) = l.split_once(' ').unwrap();

            [match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            }]
            .repeat(count.parse::<usize>().expect("Parse number of steps"))
        })
        .collect()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let moves = parse_input(input);

    // Create a rope of length 2
    let mut rope = Rope::<2>::new();

    // Keep track of visited positions
    let mut visited = HashSet::from([Knot::default()]);

    // Move rope and save tail position for every move
    for dir in moves {
        visited.insert(*rope.move_head(dir));
    }

    visited.len().to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let moves = parse_input(input);

    // Create a rope of length 10
    let mut rope = Rope::<10>::new();

    // Keep track of visited positions
    let mut visited = HashSet::from([Knot::default()]);

    // Move rope and save tail position for every move
    for dir in moves {
        visited.insert(*rope.move_head(dir));
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
