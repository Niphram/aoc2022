use std::{
    collections::{BTreeMap, HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use petgraph::{algo::dijkstra, prelude::DiGraphMap};

type PosInTime = (usize, usize, usize);

#[derive(Debug)]
struct Dijkstra {}

impl Dijkstra {
    /// Simplified Dijkstra algorithm. Returns early.
    fn solve(
        nodes: &HashSet<PosInTime>,
        start: PosInTime,
        end_fn: impl Fn(&PosInTime) -> bool,
    ) -> Option<usize> {
        // Map holds all nodes that still have to be visited and their distance from the start
        let mut nodes = nodes
            .iter()
            .map(|pos| (*pos, usize::MAX))
            .collect::<HashMap<_, _>>();

        // Set start position to a distance of 0
        nodes.insert(start, 0);

        // Repeat until all nodes are visited
        while !nodes.is_empty() {
            // Find the node with the lowest distance value, that still has to be visited
            let cur_node = nodes.iter().min_by_key(|node| node.1)?;
            let cur_pos = *cur_node.0;
            let cur_dist = *cur_node.1;

            // Check if the current node is the target
            if end_fn(&cur_pos) {
                return Some(cur_dist);
            }

            let mut visit = |pos: PosInTime| {
                // Check if the node
                // 1. Was not visited
                // 2. Is closer than before
                // 3. Passes the step_fn test
                if nodes.contains_key(&pos) && nodes[&pos] > cur_dist {
                    // Update the distance
                    nodes.insert(pos, cur_dist + 1);
                }
            };

            // Visit the four neighbors
            let (time, x, y) = cur_pos;

            visit((time + 1, x, y));
            if x > 0 {
                visit((time + 1, x - 1, y));
            }
            if y > 0 {
                visit((time + 1, x, y - 1));
            }
            visit((time + 1, x + 1, y));
            visit((time + 1, x, y + 1));

            // Remove the current node from map
            nodes.remove(&cur_pos);
        }

        None
    }
}

type Pos = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

type Blizzard = (Pos, Direction);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valley {
    blizzards: HashSet<Blizzard>,
    width: usize,
    height: usize,
}

impl FromStr for Valley {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blizzards = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices().filter_map(move |(x, d)| match d {
                    '^' => Some(((x, y), Direction::N)),
                    '>' => Some(((x, y), Direction::E)),
                    'v' => Some(((x, y), Direction::S)),
                    '<' => Some(((x, y), Direction::W)),
                    _ => None,
                })
            })
            .collect();

        let height = s.lines().count() - 1;
        let width = s.lines().next().unwrap().len() - 1;

        Ok(Self {
            blizzards,
            width,
            height,
        })
    }
}

impl Valley {
    fn move_blizzards(&mut self) {
        self.blizzards = self
            .blizzards
            .iter()
            .map(|(pos, dir)| {
                let mut pos = match dir {
                    Direction::N => (pos.0, pos.1 - 1),
                    Direction::E => (pos.0 + 1, pos.1),
                    Direction::S => (pos.0, pos.1 + 1),
                    Direction::W => (pos.0 - 1, pos.1),
                };

                if pos.0 == 0 {
                    pos.0 = self.width - 1;
                }

                if pos.1 == 0 {
                    pos.1 = self.height - 1;
                }

                if pos.0 == self.width {
                    pos.0 = 1
                }

                if pos.1 == self.height {
                    pos.1 = 1
                }

                (pos, *dir)
            })
            .collect();
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        let smaller = a.min(b);
        let larger = a.max(b);

        let mut lcm = larger;

        while (lcm % smaller) != 0 {
            lcm += larger;
        }

        lcm
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let mut valley: Valley = input.parse().unwrap();

    let cycle = lcm(valley.width - 1, valley.height - 1);

    let mut valleys = vec![];

    let start = Instant::now();

    for _ in 0..cycle {
        valley.move_blizzards();
        valleys.push(valley.clone());
    }

    println!("Simulated {} Iterations in {:?}", cycle, start.elapsed());
    let start = Instant::now();

    let mut nodes = HashSet::new();
    for time in 0..valleys.len() {
        for x in 1..valley.width {
            for y in 1..valley.height {
                nodes.insert((time, x as isize, y as isize));
            }
        }

        nodes.insert((time, 1, 0));
        nodes.insert((time, (valley.width as isize - 1), valley.height as isize));
    }

    println!("Created {} Nodes in {:?}", nodes.len(), start.elapsed());
    let start = Instant::now();

    for (time, valley) in valleys.iter().enumerate() {
        for ((x, y), _) in &valley.blizzards {
            nodes.remove(&((time + 1) % cycle, *x as isize, *y as isize));
        }
    }

    println!(
        "Removed all blizzards from nodes in {:?}. {} Nodes remaining",
        start.elapsed(),
        nodes.len()
    );

    let start = Instant::now();
    let mut graph = DiGraphMap::new();

    for node in &nodes {
        let (time, x, y) = *node;

        [(x, y), (x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter(|&p| nodes.contains(&((time + 1) % cycle, p.0, p.1)))
            .for_each(|p| {
                graph.add_edge(*node, ((time + 1) % cycle, p.0, p.1), 1);
            })
    }

    println!("Created graph in {:?}", start.elapsed());

    let res = dijkstra(&graph, (0, 1, 0), None, |_| 1);

    res.iter()
        .filter_map(|(end, value)| {
            if end.1 == (valley.width as isize) - 1 && end.2 == valley.height as isize {
                Some(value)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut valley: Valley = input.parse().unwrap();

    let cycle = lcm(valley.width - 1, valley.height - 1);

    let mut valleys = vec![];

    let start = Instant::now();

    for _ in 0..cycle {
        valley.move_blizzards();
        valleys.push(valley.clone());
    }

    println!("Simulated {} Iterations in {:?}", cycle, start.elapsed());
    let start = Instant::now();

    let mut nodes = HashSet::new();
    for time in 0..valleys.len() {
        for x in 1..valley.width {
            for y in 1..valley.height {
                nodes.insert((time, x as isize, y as isize));
            }
        }

        nodes.insert((time, 1, 0));
        nodes.insert((time, (valley.width as isize - 1), valley.height as isize));
    }

    println!("Created {} Nodes in {:?}", nodes.len(), start.elapsed());
    let start = Instant::now();

    for (time, valley) in valleys.iter().enumerate() {
        for ((x, y), _) in &valley.blizzards {
            nodes.remove(&((time + 1) % cycle, *x as isize, *y as isize));
        }
    }

    println!(
        "Removed all blizzards from nodes in {:?}. {} Nodes remaining",
        start.elapsed(),
        nodes.len()
    );

    let start = Instant::now();
    let mut graph = DiGraphMap::new();

    for node in &nodes {
        let (time, x, y) = *node;

        [(x, y), (x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter(|&p| nodes.contains(&((time + 1) % cycle, p.0, p.1)))
            .for_each(|p| {
                graph.add_edge(*node, ((time + 1) % cycle, p.0, p.1), 1);
            })
    }

    println!("Created graph in {:?}", start.elapsed());

    let trip_1 = dijkstra(&graph, (0, 1, 0), None, |_| 1)
        .into_iter()
        .filter(|(end, ..)| end.1 == (valley.width as isize) - 1 && end.2 == valley.height as isize)
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let trip_2 = dijkstra(&graph, trip_1.0, None, |_| 1)
        .into_iter()
        .filter(|(end, ..)| end.1 == 1 && end.2 == 0)
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let trip_3 = dijkstra(&graph, trip_2.0, None, |_| 1)
        .into_iter()
        .filter(|(end, ..)| end.1 == (valley.width as isize) - 1 && end.2 == valley.height as isize)
        .min_by_key(|(_, v)| *v)
        .unwrap();

    (trip_1.1 + trip_2.1 + trip_3.1).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 23");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
