use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Default)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl Pos {
    /// Create new Pos
    fn new(x: usize, y: usize, z: usize) -> Self {
        Pos { x, y, z }
    }

    /// elementwise maximum
    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s
            .split(',')
            .map(&str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ())?;

        Ok(Pos {
            x: parsed[0],
            y: parsed[1],
            z: parsed[2],
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Material {
    Air,
    OutsideAir,
    Lava,
}

#[derive(Debug)]
struct Grid3D {
    data: Vec<Material>,
    width: usize,
    height: usize,
    depth: usize,
}

impl Grid3D {
    /// Create new grid with the given size
    fn new(width: usize, height: usize, depth: usize) -> Self {
        let data_len = width * height * depth;

        Self {
            data: vec![Material::Air; data_len],
            width,
            height,
            depth,
        }
    }

    /// Return an iterator for all neighboring cells
    fn neighbors(&self, &Pos { x, y, z }: &Pos) -> impl Iterator<Item = Pos> {
        [
            (x < self.width - 1).then_some(Pos::new(x + 1, y, z)),
            (y < self.height - 1).then_some(Pos::new(x, y + 1, z)),
            (z < self.depth - 1).then_some(Pos::new(x, y, z + 1)),
            x.checked_sub(1).map(|x| Pos::new(x, y, z)),
            y.checked_sub(1).map(|y| Pos::new(x, y, z)),
            z.checked_sub(1).map(|z| Pos::new(x, y, z)),
        ]
        .into_iter()
        .flatten()
    }

    /// Count the number of neighboring cells with the material
    fn count_neighbors(&self, pos: &Pos, mat: Material) -> usize {
        self.neighbors(pos)
            .map(|n| self[&n])
            .filter(|m| *m == mat)
            .count()
    }
}

impl FromStr for Grid3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .lines()
            .map(&str::parse::<Pos>)
            .collect::<Result<Vec<_>, _>>()?;

        // Find the bounds (+3 to make sure the droplet is surrounded by air)
        let Pos {
            x: width,
            y: height,
            z: depth,
        } = positions.iter().fold(Pos::default(), |a, b| a.max(b));

        let mut grid = Grid3D::new(width + 3, height + 3, depth + 3);

        // Add all positions to grid
        for pos in positions {
            // +1 to make sure the droplet is surrounded by air
            let pos = Pos::new(pos.x + 1, pos.y + 1, pos.z + 1);
            grid[&pos] = Material::Lava;
        }

        Ok(grid)
    }
}

impl Index<&Pos> for Grid3D {
    type Output = Material;

    fn index(&self, Pos { x, y, z }: &Pos) -> &Self::Output {
        &self.data[z * self.height * self.width + y * self.width + x]
    }
}

impl IndexMut<&Pos> for Grid3D {
    fn index_mut(&mut self, Pos { x, y, z }: &Pos) -> &mut Self::Output {
        &mut self.data[z * self.height * self.width + y * self.width + x]
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let grid: Grid3D = input.parse().unwrap();

    let mut area = 0;

    // Check every position in the grid
    for z in 0..grid.depth {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let pos = Pos::new(x, y, z);

                // Check if it is air
                if grid[&pos] == Material::Air {
                    // Count the lava-tiles next to it
                    area += grid.count_neighbors(&pos, Material::Lava);
                }
            }
        }
    }

    area.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let mut grid: Grid3D = input.parse().unwrap();

    // Flood fill outside
    let mut queue = vec![Pos::default()];

    while let Some(pos) = queue.pop() {
        if grid[&pos] == Material::Air {
            grid[&pos] = Material::OutsideAir;

            for n in grid.neighbors(&pos) {
                queue.push(n);
            }
        }
    }

    let mut area = 0;

    // Check every position in the grid
    for z in 0..grid.depth {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let pos = Pos::new(x, y, z);

                // Check if it is outside air (produced in the flood-fill above)
                if grid[&pos] == Material::OutsideAir {
                    // Count the lava-tiles next to it
                    area += grid.count_neighbors(&pos, Material::Lava);
                }
            }
        }
    }

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
