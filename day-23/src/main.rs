use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
    num::TryFromIntError,
    ops::Add,
    str::FromStr,
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Elves {
    positions: BTreeSet<Pos>,
    iteration: usize,
}

impl FromStr for Elves {
    type Err = TryFromIntError;

    /// Will fail, if the grid is larger than `isize::MAX` in any direction
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
                    .map(|(x, y)| {
                        let x = x.try_into()?;
                        let y = y.try_into()?;
                        Ok(Pos::new(x, y))
                    })
            })
            .collect::<Result<BTreeSet<Pos>, TryFromIntError>>()?;

        Ok(Self {
            positions,
            iteration: 0,
        })
    }
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.get_aabb();

        writeln!(f, "== End of Round {} ==", self.iteration)?;

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if self.positions.contains(&Pos::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Elves {
    /// Simulates one round and returns if the elves have moved
    fn iterate(&mut self) -> bool {
        // The checks for the different sides
        const CHECKS: [[Pos; 3]; 4] = [
            [Pos::new(-1, -1), Pos::new(0, -1), Pos::new(1, -1)],
            [Pos::new(1, 1), Pos::new(0, 1), Pos::new(-1, 1)],
            [Pos::new(-1, 1), Pos::new(-1, 0), Pos::new(-1, -1)],
            [Pos::new(1, -1), Pos::new(1, 0), Pos::new(1, 1)],
        ];

        let proposed_positions = self
            .positions
            .iter()
            .map(|pos| {
                // Execute all checks for this elf
                let checked_sides =
                    CHECKS
                        .map(|offsets| offsets.map(|o| o + *pos))
                        .map(|[l, f, r]| {
                            (!(self.positions.contains(&l)
                                || self.positions.contains(&f)
                                || self.positions.contains(&r)))
                            .then_some(f)
                        });

                // If the elf could move in all directions, don't move (no neighbors)
                let new_pos = if checked_sides.iter().all(Option::is_some) {
                    *pos
                } else if let Some(new_pos) = checked_sides[self.iteration % 4] {
                    // Check all four rules
                    new_pos
                } else if let Some(new_pos) = checked_sides[(self.iteration + 1) % 4] {
                    // Check all four rules
                    new_pos
                } else if let Some(new_pos) = checked_sides[(self.iteration + 2) % 4] {
                    // Check all four rules
                    new_pos
                } else if let Some(new_pos) = checked_sides[(self.iteration + 3) % 4] {
                    // Check all four rules
                    new_pos
                } else {
                    // No rules match, don't move
                    *pos
                };

                (pos, new_pos)
            })
            .collect::<HashMap<_, _>>();

        // Move elves if possible
        let new_positions = proposed_positions
            .iter()
            .map(|(&pos, new_pos)| {
                // Check if more than one elf want's to move to this position
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

        // Check of some elves have moved
        let some_elves_moved = self.positions != new_positions;

        // Update positions
        self.positions = new_positions;

        // Update iterations
        self.iteration += 1;

        some_elves_moved
    }

    /// Return the minimum bounding-box
    fn get_aabb(&self) -> (Pos, Pos) {
        let max_x = self.positions.iter().map(|p| p.x).max().unwrap();
        let min_x = self.positions.iter().map(|p| p.x).min().unwrap();
        let max_y = self.positions.iter().map(|p| p.y).max().unwrap();
        let min_y = self.positions.iter().map(|p| p.y).min().unwrap();

        (Pos::new(min_x, min_y), Pos::new(max_x, max_y))
    }

    // Return the amount of elves
    fn count(&self) -> usize {
        self.positions.len()
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let mut elves: Elves = input.parse().unwrap();

    // Iterate 10 times
    for _ in 0..10 {
        elves.iterate();
    }

    // Calculate area of bounding-box
    let (min, max) = elves.get_aabb();
    let area = (min.x.abs_diff(max.x) + 1) * (min.y.abs_diff(max.y) + 1);

    // Subtract elves from area
    (area - elves.count()).to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut elves: Elves = input.parse().unwrap();

    // Iterate forever
    for iteration in 0.. {
        if !elves.iterate() {
            // We're done, if no elves have moved this iteration w
            return (iteration + 1).to_string();
        }
    }

    unreachable!()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 23");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let start_time = Instant::now();
    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res} - Took {:?}", start_time.elapsed());
}
