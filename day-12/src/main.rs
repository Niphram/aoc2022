use std::{collections::HashMap, ops::Index};

type Pos = (usize, usize);

#[derive(Debug)]
struct Dijkstra<T>
where
    T: Sized,
{
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Index<Pos> for Dijkstra<T> {
    type Output = T;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T> Dijkstra<T> {
    fn new(data: Vec<T>, height: usize) -> Self {
        let width = data.len() / height;

        Dijkstra {
            data,
            width,
            height,
        }
    }

    /// Simplified Dijkstra algorithm. Returns early.
    fn solve(
        &self,
        start: Pos,
        end_fn: impl Fn(&Pos, &T) -> bool,
        step_fn: impl Fn(&T, &T) -> bool,
    ) -> Option<usize> {
        // Map holds all nodes that still have to be visited and their distance from the start
        let mut nodes = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                // Set distance for every node to the max value
                nodes.insert((x, y), usize::MAX);
            }
        }

        // Set start position to a distance of 0
        nodes.insert(start, 0);

        // Repeat until all nodes are visited
        while !nodes.is_empty() {
            // Find the node with the lowest distance value, that still has to be visited
            let cur_node = nodes.iter().min_by_key(|node| node.1)?;
            let cur_pos = *cur_node.0;
            let cur_dist = *cur_node.1;

            // Check if the current node is the target
            if end_fn(&cur_pos, &self[cur_pos]) {
                return Some(cur_dist);
            }

            let mut visit = |pos: Pos| {
                // Check if the node
                // 1. Was not visited
                // 2. Is closer than before
                // 3. Passes the step_fn test
                if nodes.contains_key(&pos)
                    && nodes[&pos] > cur_dist
                    && step_fn(&self[cur_pos], &self[pos])
                {
                    // Update the distance
                    nodes.insert(pos, cur_dist + 1);
                }
            };

            // Visit the four neighbors
            let (x, y) = cur_pos;

            if x > 0 {
                visit((x - 1, y));
            }
            if y > 0 {
                visit((x, y - 1));
            }
            if x < self.width - 1 {
                visit((x + 1, y));
            }
            if y < self.height - 1 {
                visit((x, y + 1));
            }

            // Remove the current node from map
            nodes.remove(&cur_pos);
        }

        None
    }
}

fn parse_input(input: &str) -> (Dijkstra<u8>, Pos, Pos) {
    let lines: Vec<_> = input.lines().collect();

    let height = lines.len();

    let start = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.chars().position(|c| c == 'S').map(|x| (x, y)))
        .unwrap();

    let end = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.chars().position(|c| c == 'E').map(|x| (x, y)))
        .unwrap();

    let heightmap = lines
        .iter()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => b'a',
                    'E' => b'z',
                    c => c as u8,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (Dijkstra::new(heightmap, height), start, end)
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let (grid, start, end) = parse_input(input);

    // Find the shortest distance from start to end
    let res = grid.solve(start, |pos, _| pos == &end, |cur, next| *next <= cur + 1);

    res.unwrap().to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let (grid, _, end) = parse_input(input);

    // Find the shortest distance from end to any 'a'
    // Inverted the step_fn
    let res = grid.solve(
        end,
        |_, height| *height == b'a',
        |cur, next| *cur <= next + 1,
    );

    res.unwrap().to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 11");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
