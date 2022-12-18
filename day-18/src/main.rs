use std::collections::HashMap;

type Pos = (usize, usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Material {
    Air,
    Lava,
}

#[derive(Debug)]
struct Grid3D {
    grid: HashMap<Pos, Material>,
    surface_area: usize,
    size: (usize, usize, usize),
}

impl Grid3D {
    fn new(size: (usize, usize, usize)) -> Self {
        Self {
            grid: HashMap::new(),
            surface_area: 0,
            size,
        }
    }

    fn neighbors(&self, &(x, y, z): &Pos) -> impl Iterator<Item = Pos> {
        [
            (x < self.size.0).then_some((x + 1, y, z)),
            (y < self.size.1).then_some((x, y + 1, z)),
            (z < self.size.2).then_some((x, y, z + 1)),
            x.checked_sub(1).map(|x| (x, y, z)),
            y.checked_sub(1).map(|y| (x, y, z)),
            z.checked_sub(1).map(|z| (x, y, z)),
        ]
        .into_iter()
        .flatten()
    }

    fn count_neighbors(&self, pos: &Pos, mat: Material) -> usize {
        self.neighbors(pos)
            .filter_map(|n| self.grid.get(&n))
            .filter(|&m| *m == mat)
            .count()
    }

    fn put(&mut self, pos: &Pos, mat: Material) {
        self.grid.insert(*pos, mat);

        let neighbors = self.count_neighbors(pos, mat);

        self.surface_area += 6;
        self.surface_area -= neighbors * 2;
    }

    fn get(&self, pos: &Pos) -> Option<&Material> {
        self.grid.get(pos)
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let cubes = input
        .lines()
        .filter_map(|line| {
            let mut coords = line
                .split(',')
                .map(&str::parse::<usize>)
                .filter_map(Result::ok);

            Some((coords.next()?, coords.next()?, coords.next()?))
        })
        .collect::<Vec<Pos>>();

    let size = cubes
        .iter()
        .fold((0, 0, 0), |a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)));

    let mut grid = Grid3D::new(size);

    for cube in &cubes {
        grid.put(cube, Material::Lava);
    }

    grid.surface_area.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let cubes = input
        .lines()
        .filter_map(|line| {
            let mut coords = line
                .split(',')
                .map(&str::parse::<usize>)
                .filter_map(Result::ok);

            Some((coords.next()? + 1, coords.next()? + 1, coords.next()? + 1))
        })
        .collect::<Vec<Pos>>();

    let size = cubes
        .iter()
        .fold((0, 0, 0), |a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)));

    let mut grid = Grid3D::new((size.0 + 1, size.1 + 1, size.2 + 1));

    for cube in &cubes {
        grid.put(cube, Material::Lava);
    }

    // Flood fill outside
    let mut queue = vec![(0, 0, 0)];

    while let Some(pos) = queue.pop() {
        if grid.get(&pos).is_none() {
            grid.put(&pos, Material::Air);
            for n in grid.neighbors(&pos) {
                queue.push(n);
            }
        }
    }

    // Get exposed lava
    let area: usize = grid
        .grid
        .iter()
        .filter(|(_, &mat)| mat == Material::Air)
        .map(|(pos, _)| grid.count_neighbors(pos, Material::Lava))
        .sum();

    area.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 18");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
