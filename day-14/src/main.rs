#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Simulation {
    grid: Vec<Vec<Material>>,
    width: usize,
    height: usize,
}

impl Simulation {
    fn new(width: usize, height: usize) -> Self {
        let grid = (0..=height)
            .map(|_| [Material::Air].repeat(width + 1))
            .collect();

        Self {
            grid,
            width,
            height,
        }
    }

    fn draw_line(&mut self, material: Material, line: &[(isize, isize)]) {
        for points in line.windows(2) {
            let mut start = points[0];
            let end = points[1];

            while start != end {
                self.set_point(material, &start);

                start.0 += (end.0 - start.0).signum();
                start.1 += (end.1 - start.1).signum();
            }
        }
        self.set_point(material, line.last().unwrap());
    }

    fn set_point(&mut self, material: Material, pos: &(isize, isize)) {
        self.grid[pos.1 as usize][pos.0 as usize] = material;
    }

    fn get_point(&self, pos: &(isize, isize)) -> Option<Material> {
        self.grid
            .get(pos.1 as usize)
            .and_then(|row| row.get(pos.0 as usize))
            .copied()
    }

    fn simulate_sand(&mut self, pos: &(isize, isize)) -> Option<(isize, isize)> {
        let mut pos = *pos;

        loop {
            if self.get_point(&(pos.0, pos.1 + 1))? == Material::Air {
                pos = (pos.0, pos.1 + 1);
            } else if self.get_point(&(pos.0 - 1, pos.1 + 1))? == Material::Air {
                pos = (pos.0 - 1, pos.1 + 1);
            } else if self.get_point(&(pos.0 + 1, pos.1 + 1))? == Material::Air {
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                self.set_point(Material::Sand, &pos);
                break;
            }
        }

        Some(pos)
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let lines: Vec<Vec<(isize, isize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').expect("Split point into x and y");

                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let max_x = lines
        .iter()
        .map(|points| points.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap();

    let max_y = lines
        .iter()
        .map(|points| points.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap();

    let mut sim = Simulation::new(*max_x as usize, *max_y as usize);

    for line in lines {
        sim.draw_line(Material::Rock, &line);
    }

    let mut count = 0;

    while let Some(_) = sim.simulate_sand(&(500, 0)) {
        count += 1;
    }

    count.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let lines: Vec<Vec<(isize, isize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').expect("Split point into x and y");

                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let max_x = lines
        .iter()
        .map(|points| points.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap()
        + 500;

    let max_y = lines
        .iter()
        .map(|points| points.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap()
        + 2;

    dbg!(max_y);

    let mut sim = Simulation::new(max_x as usize, max_y as usize);

    for line in lines {
        sim.draw_line(Material::Rock, &line);
    }

    sim.draw_line(Material::Rock, &[(0, max_y), (max_x, max_y)]);

    let mut count = 0;

    while let Some(pos) = sim.simulate_sand(&(500, 0)) {
        count += 1;

        if pos == (500, 0) {
            println!("Stuck!");
            break;
        }
    }

    count.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 14");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
