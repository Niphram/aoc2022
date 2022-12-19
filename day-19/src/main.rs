use std::{str::FromStr, time::Instant, vec};

use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore_robot_cost: Materials,
    clay_robot_cost: Materials,
    obsidian_robot_cost: Materials,
    geode_robot_cost: Materials,
}

#[derive(Debug, Clone, Copy, Default)]
struct Materials {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

#[derive(Debug, Clone, Copy)]
struct Robots {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}

impl Materials {
    fn new(ore: usize, clay: usize, obsidian: usize, geodes: usize) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geodes,
        }
    }

    fn check(&self, cost: &Self) -> bool {
        self.ore >= cost.ore
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
            && self.geodes >= cost.geodes
    }

    fn sub(&mut self, cost: &Self) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
        self.geodes -= cost.geodes;
    }
}

impl Default for Robots {
    fn default() -> Self {
        Self {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(&str::parse::<usize>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(Self {
            ore_robot_cost: Materials::new(numbers[0], 0, 0, 0),
            clay_robot_cost: Materials::new(numbers[1], 0, 0, 0),
            obsidian_robot_cost: Materials::new(numbers[2], numbers[3], 0, 0),
            geode_robot_cost: Materials::new(numbers[4], 0, numbers[5], 0),
        })
    }
}

fn run(materials: &mut Materials, robots: &Robots) {
    materials.ore += robots.ore_robot;
    materials.clay += robots.clay_robot;
    materials.obsidian += robots.obsidian_robot;
    materials.geodes += robots.geode_robot;
}

fn optimize(time: usize, materials: &Materials, robots: &Robots, blueprint: &Blueprint) -> usize {
    if time.checked_sub(1).is_none() {
        return materials.geodes;
    }

    let mut new_states = vec![];

    if materials.check(&blueprint.geode_robot_cost) {
        let mut materials = *materials;
        materials.sub(&blueprint.geode_robot_cost);
        let mut robots = *robots;
        robots.geode_robot += 1;
        new_states.push((materials, robots))
    } else if materials.check(&blueprint.obsidian_robot_cost)
        && robots.obsidian_robot < blueprint.geode_robot_cost.obsidian
    {
        let mut materials = *materials;
        materials.sub(&blueprint.obsidian_robot_cost);
        let mut robots = *robots;
        robots.obsidian_robot += 1;
        new_states.push((materials, robots))
    } else if materials.check(&blueprint.clay_robot_cost)
        && robots.clay_robot < blueprint.obsidian_robot_cost.clay
    {
        let mut materials = *materials;
        materials.sub(&blueprint.clay_robot_cost);
        let mut robots = *robots;
        robots.clay_robot += 1;
        new_states.push((materials, robots))
    }

    if materials.check(&blueprint.ore_robot_cost)
        && robots.ore_robot
            < blueprint
                .clay_robot_cost
                .ore
                .max(blueprint.obsidian_robot_cost.ore)
                .max(blueprint.geode_robot_cost.ore)
    {
        let mut materials = *materials;
        materials.sub(&blueprint.ore_robot_cost);
        let mut robots = *robots;
        robots.ore_robot += 1;
        new_states.push((materials, robots))
    }

    new_states.push((*materials, *robots));

    // Tick state
    new_states
        .par_iter_mut()
        .map(|(mat, new_bots)| {
            run(mat, robots);
            optimize(time - 1, mat, new_bots, blueprint)
        })
        .max()
        .unwrap()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .map(&str::parse)
        .filter_map(Result::ok)
        .collect();

    let res = blueprints
        .par_iter()
        .enumerate()
        .map(|(idx, blueprint)| {
            let res =
                optimize(24, &Materials::default(), &Robots::default(), blueprint) * (idx + 1);
            println!("Finished Blueprint {}; Quality Level = {}", idx + 1, res);
            res
        })
        .sum::<usize>();

    res.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .take(3)
        .map(&str::parse)
        .filter_map(Result::ok)
        .collect();

    let res = blueprints
        .par_iter()
        .enumerate()
        .map(|(idx, blueprint)| {
            let res = optimize(32, &Materials::default(), &Robots::default(), blueprint);
            println!("Finished Blueprint {}; Maximum Geodes = {}", idx + 1, res);
            res
        })
        .product::<usize>();

    res.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 19");
    println!("!!! THIS WILL TAKE A LOOOONG TIME !!!");

    let start = Instant::now();
    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}. Took {:?}", start.elapsed());

    let start = Instant::now();
    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}. Took {:?}", start.elapsed());
}
