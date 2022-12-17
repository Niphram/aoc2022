use std::{collections::HashSet, fmt::Display, ops::SubAssign, str::FromStr};

type Pos = (usize, usize);

#[derive(Debug)]
struct Rock {
    shape: HashSet<Pos>,
}

impl FromStr for Rock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count() - 1;

        let shape = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(|(x, c)| (c == '#').then_some((x, height - y)))
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Rock { shape })
    }
}

#[derive(Debug)]
struct Stack {
    grid: Vec<[bool; 7]>,
}

impl Stack {
    fn new() -> Self {
        Stack { grid: vec![] }
    }

    /// Checks if the position has a rock or is outside of the valid area
    fn get(&self, pos: &Pos) -> bool {
        if !(0..7).contains(&pos.0) {
            return true;
        }

        self.grid
            .get(pos.1)
            .map(|&row| row[pos.0])
            .unwrap_or_default()
    }

    fn set(&mut self, pos: &Pos) {
        if (0..7).contains(&pos.0) {
            while pos.1 >= self.grid.len() {
                self.grid.push([false; 7]);
            }

            if let Some(row) = self.grid.get_mut(pos.1) {
                row[pos.0] = true;
            }
        }
    }

    /// Returns true, if some part of the rock collides
    fn rock_collision(&self, pos: &Pos, rock: &Rock) -> bool {
        rock.shape.iter().any(|rock| {
            let pos = (rock.0 + pos.0, rock.1 + pos.1);
            self.get(&pos)
        })
    }

    /// Draws a rock at the specified position and returns the number of new lines
    fn draw_rock(&mut self, pos: &Pos, rock: &Rock) -> usize {
        let old_height = self.height();

        rock.shape.iter().for_each(|rock| {
            let pos = (rock.0 + pos.0, rock.1 + pos.1);
            self.set(&pos);
        });

        self.height().saturating_sub(old_height)
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter().rev() {
            let line = line
                .map(|r| match r {
                    true => '#',
                    false => '.',
                })
                .iter()
                .collect::<String>();

            writeln!(f, "|{}|", line)?;
        }

        writeln!(f, "+-------+")?;

        Ok(())
    }
}

/// Compute the solution to part 1
fn part_1(input: &str, rocks: &Vec<Rock>) -> String {
    // Create infinite iterator from input
    let mut wind_pattern = input.chars().cycle();

    // Create new stack
    let mut stack = Stack::new();

    // Drop 2022 rocks
    for idx in 0..2022 {
        // Rocks always start at x = 2 and y = 3 higher than the highest rock
        let mut pos: Pos = (2, stack.height() + 3);

        // Select rock
        let rock = &rocks[idx % rocks.len()];

        // Repeat until rock is dropped
        loop {
            // Shift position left or right
            let new_pos: Pos = match wind_pattern.next() {
                Some('<') => (pos.0.saturating_sub(1), pos.1),
                Some('>') => (pos.0 + 1, pos.1),
                _ => panic!(),
            };

            // Check for collisions
            if !stack.rock_collision(&new_pos, rock) {
                pos = new_pos;
            }

            // If the rock can drop
            if pos.1 > 0 {
                // Shift one down
                let new_pos = (pos.0, pos.1 - 1);

                // Check for collision
                if !stack.rock_collision(&new_pos, rock) {
                    pos = new_pos;
                } else {
                    // Break if rock can't move down
                    break;
                }
            } else {
                // Break if rock has reached the bottom
                break;
            }
        }

        // Insert rock into stack
        stack.draw_rock(&pos, rock);
    }

    stack.height().to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str, rocks: &Vec<Rock>) -> String {
    // Create infinite iterator from input
    let mut wind_pattern = input.chars().enumerate().cycle();

    // Create new stack
    let mut stack = Stack::new();

    // Keep track of started cycles ()
    let mut wind_cycles = vec![None; input.len()];

    // Keep track of height changes for every rock
    let mut height_deltas = vec![];

    // Keep track of how many times we found a cycle
    let mut matched_cycle = 0;

    // Number of rocks to drop
    const ROCKS: usize = 1000000000000;

    for idx in 0..ROCKS {
        let rock_idx = idx % rocks.len();

        let mut pos: Pos = (2, stack.height() + 3);

        let rock = &rocks[rock_idx];

        // Find out on which wind the piece was dropped
        let last_wind_idx = loop {
            let wind = wind_pattern.next().unwrap();

            let new_pos: Pos = match wind.1 {
                '<' => (pos.0.saturating_sub(1), pos.1),
                '>' => (pos.0 + 1, pos.1),
                _ => panic!(),
            };

            if !stack.rock_collision(&new_pos, rock) {
                pos = new_pos;
            }

            if pos.1 > 0 {
                let new_pos = (pos.0, pos.1 - 1);

                if !stack.rock_collision(&new_pos, rock) {
                    pos = new_pos;
                } else {
                    break wind.0;
                }
            } else {
                break wind.0;
            }
        };

        // Put rock on stack and push height change
        let new_rows = stack.draw_rock(&pos, rock);
        height_deltas.push(new_rows);

        // If we just dropped the first rock of the rock cycle
        if rock_idx == 0 {
            // Check if we already dropped the first rock in this specific wind
            let position = &mut wind_cycles[last_wind_idx];

            match *position {
                None => {
                    // If not, reset matched_cycle and remember this index
                    matched_cycle = 0;
                    *position = Some(idx);
                }
                Some(first_idx) => {
                    // Increase matched_cycle
                    matched_cycle += 1;

                    // if multiple cycles were matched in a row
                    if matched_cycle > 1 {
                        // The length of the cycle
                        let cycle_len = idx - first_idx;

                        // The number of rocks we still have to drop
                        let left_to_drop = ROCKS - idx - 1;

                        // Take cycle_len height deltas from the end
                        let heights_cycle = &height_deltas[height_deltas.len() - cycle_len..];

                        // Sum all those heights to get the height change per cycle
                        let height_per_cycle: usize = heights_cycle.iter().sum();

                        let incomplete_cycle_height: usize =
                            heights_cycle.iter().take(left_to_drop % cycle_len).sum();

                        // Result = current height + full cycles * height per cycle + incomplete cycle
                        return (stack.height()
                            + (left_to_drop / cycle_len * height_per_cycle)
                            + incomplete_cycle_height)
                            .to_string();
                    }
                }
            }
        }
    }

    stack.height().to_string()
}

fn main() {
    let input = include_str!("input.txt");

    let rocks_string = "####\n
.#.\n###\n.#.\n
..#\n..#\n###\n
#\n#\n#\n#\n
##\n##";

    let rocks: Vec<Rock> = rocks_string
        .split("\n\n")
        .filter_map(|rock| rock.parse().ok())
        .collect();

    println!("Advent of Code 2022 - Day 17");

    let part_1 = part_1(input, &rocks);
    println!("Part 1: {part_1}");

    let part_2 = part_2(input, &rocks);
    println!("Part 2: {part_2}");
}
