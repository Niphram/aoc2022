use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Height {
    Height(usize),
    Start,
    End,
}

enum Node {
    Unvisited,
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let heightmap: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    ('a'..='z')
                        .position(|t| t == c)
                        .map(Height::Height)
                        .or_else(|| match c {
                            'S' => Some(Height::Start),
                            'E' => Some(Height::End),
                            _ => panic!("Unknown elevation"),
                        })
                })
                .collect()
        })
        .collect();

    let start = heightmap
        .iter()
        .enumerate()
        .find_map(|(row, data)| {
            data.iter()
                .position(|h| *h == Height::Start)
                .map(|col| (col, row))
        })
        .unwrap();

    let end = heightmap
        .iter()
        .enumerate()
        .find_map(|(row, data)| {
            data.iter()
                .position(|h| *h == Height::End)
                .map(|col| (col, row))
        })
        .unwrap();

    // Dijkstra

    let mut dist = HashMap::new();
    let mut unvisited = HashSet::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            unvisited.insert((x, y));
        }
    }
    dist.insert(start, 0);

    let get_height = |h: Height| match h {
        Height::Start => 0,
        Height::End => 25,
        Height::Height(h) => h,
    };

    while !unvisited.is_empty() {
        let closest = dist
            .iter()
            .filter(|node| unvisited.contains(node.0))
            .min_by_key(|node| node.1);

        if !unvisited.contains(&end) || closest.is_none() {
            break;
        }

        let (x, y) = *closest.unwrap().0;

        let current_height = get_height(heightmap[y][x]);
        let current_dist = *dist.get(&(x, y)).unwrap();

        let mut visit = |x: usize, y: usize| {
            if get_height(heightmap[y][x]) <= current_height + 1 && unvisited.contains(&(x, y)) {
                let old = dist.get(&(x, y));

                if old.is_none() || *old.unwrap() > current_dist {
                    dist.insert((x, y), current_dist + 1);
                }
            }
        };

        if x > 0 {
            visit(x - 1, y);
        }
        if y > 0 {
            visit(x, y - 1);
        }
        if x < heightmap[0].len() - 1 {
            visit(x + 1, y);
        }
        if y < heightmap.len() - 1 {
            visit(x, y + 1);
        }

        unvisited.remove(&(x, y));
    }

    dist.get(&end).unwrap().to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let heightmap: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    ('a'..='z')
                        .position(|t| t == c)
                        .map(Height::Height)
                        .or_else(|| match c {
                            'S' => Some(Height::Start),
                            'E' => Some(Height::End),
                            _ => panic!("Unknown elevation"),
                        })
                })
                .collect()
        })
        .collect();

    let mut starts = HashSet::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            match heightmap[y][x] {
                Height::Start => {
                    starts.insert((x, y));
                }
                Height::Height(0) => {
                    starts.insert((x, y));
                }
                _ => {}
            }
        }
    }

    let end = heightmap
        .iter()
        .enumerate()
        .find_map(|(row, data)| {
            data.iter()
                .position(|h| *h == Height::End)
                .map(|col| (col, row))
        })
        .unwrap();

    // Dijkstra

    let mut dist = HashMap::new();
    let mut unvisited = HashSet::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            unvisited.insert((x, y));
        }
    }
    dist.insert(end, 0);

    let get_height = |h: Height| match h {
        Height::Start => 0,
        Height::End => 25,
        Height::Height(h) => h,
    };

    while !unvisited.is_empty() {
        let closest = dist
            .iter()
            .filter(|node| unvisited.contains(node.0))
            .min_by_key(|node| node.1);

        if closest.is_none() {
            break;
        }

        let (x, y) = *closest.unwrap().0;

        let current_height = get_height(heightmap[y][x]);
        let current_dist = *dist.get(&(x, y)).unwrap();

        let mut visit = |x: usize, y: usize| {
            if current_height <= get_height(heightmap[y][x]) + 1 && unvisited.contains(&(x, y)) {
                let old = dist.get(&(x, y));

                if old.is_none() || *old.unwrap() > current_dist {
                    if get_height(heightmap[y][x]) == 0 {
                        println!("New Start: {}", current_dist);
                    }

                    dist.insert((x, y), current_dist + 1);
                }
            }
        };

        if x > 0 {
            visit(x - 1, y);
        }
        if y > 0 {
            visit(x, y - 1);
        }
        if x < heightmap[0].len() - 1 {
            visit(x + 1, y);
        }
        if y < heightmap.len() - 1 {
            visit(x, y + 1);
        }

        unvisited.remove(&(x, y));
    }

    starts
        .iter()
        .filter_map(|start| dist.get(start))
        .min()
        .unwrap()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 11");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
