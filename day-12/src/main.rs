use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Height {
    Value(usize),
    Start,
    End,
}

impl Height {
    fn height(&self) -> usize {
        match self {
            Height::Value(v) => *v,
            Height::Start => 0,
            Height::End => 25,
        }
    }
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Grid<T: Sized> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<Height> {
    fn new(data: Vec<Height>, height: usize) -> Self {
        let width = data.len() / height;

        Grid {
            data,
            width,
            height,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn start(&self) -> Pos {
        let idx = self
            .data
            .iter()
            .position(|pos| *pos == Height::Start)
            .unwrap();

        (idx % self.width, idx / self.width)
    }

    fn end(&self) -> Pos {
        let idx = self
            .data
            .iter()
            .position(|pos| *pos == Height::End)
            .unwrap();

        (idx % self.width, idx / self.width)
    }

    fn solve(
        &self,
        start: Pos,
        end_fn: impl Fn(Pos, Height) -> bool,
        step_fn: impl Fn(Height, Height) -> bool,
    ) -> Option<usize> {
        let mut dist = HashMap::new();
        let mut unvisited = HashSet::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                unvisited.insert((x, y));
            }
        }

        dist.insert(start, 0);

        while !unvisited.is_empty() {
            let closest = dist
                .iter()
                .filter(|node| unvisited.contains(node.0))
                .min_by_key(|node| node.1)?;

            let (x, y) = *closest.0;

            if end_fn((x, y), self[(x, y)]) {
                return Some(dist[&(x, y)]);
            }

            let current_height = self[(x, y)];
            let current_dist = *dist.get(&(x, y)).unwrap();

            let mut visit = |x: usize, y: usize| {
                if step_fn(current_height, self[(x, y)]) && unvisited.contains(&(x, y)) {
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
            if x < self.width() - 1 {
                visit(x + 1, y);
            }
            if y < self.height() - 1 {
                visit(x, y + 1);
            }

            unvisited.remove(&(x, y));
        }

        None
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

fn parse_input(input: &str) -> Grid<Height> {
    let heightmap: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    ('a'..='z')
                        .position(|t| t == c)
                        .map(Height::Value)
                        .or_else(|| match c {
                            'S' => Some(Height::Start),
                            'E' => Some(Height::End),
                            _ => panic!("Unknown elevation"),
                        })
                })
                .collect()
        })
        .collect();

    let height = heightmap.len();

    Grid::new(heightmap.into_iter().flatten().collect(), height)
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let grid = parse_input(input);

    let res = grid.solve(
        grid.start(),
        |pos, _| pos == grid.end(),
        |cur, next| next.height() <= cur.height() + 1,
    );

    res.unwrap().to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let grid = parse_input(input);

    let res = grid.solve(
        grid.end(),
        |_, height| height.height() == 0,
        |cur, next| cur.height() <= next.height() + 1,
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
