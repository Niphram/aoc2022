/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    fn tree_visible(grid: &Vec<Vec<i32>>, (tree_x, tree_y): (usize, usize)) -> bool {
        let tree = grid[tree_y][tree_x];

        let (max_x, max_y) = (grid[0].len(), grid.len());

        for x in (0..tree_x).rev() {
            if tree <= grid[tree_y][x] {
                for x in (tree_x + 1)..max_x {
                    if tree <= grid[tree_y][x] {
                        for y in (0..tree_y).rev() {
                            if tree <= grid[y][tree_x] {
                                for y in (tree_y + 1)..max_y {
                                    if tree <= grid[y][tree_x] {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        true
    }

    let visible: usize = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(x, _)| tree_visible(&grid, (*x, y)))
                .count()
        })
        .sum();

    visible.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    fn calc_scenic_score(grid: &Vec<Vec<i32>>, (tree_x, tree_y): (usize, usize)) -> usize {
        let (max_x, max_y) = (grid[0].len() - 1, grid.len() - 1);

        if tree_x == 0 || tree_x == max_x || tree_y == 0 || tree_y == max_y {
            return 0;
        }

        let tree = grid[tree_y][tree_x];

        let mut score = 1;

        score *= (1..tree_x)
            .rev()
            .take_while(|x| tree > grid[tree_y][*x])
            .count()
            + 1;

        score *= ((tree_x + 1)..(max_x))
            .take_while(|x| tree > grid[tree_y][*x])
            .count()
            + 1;

        score *= (1..tree_y)
            .rev()
            .take_while(|y| tree > grid[*y][tree_x])
            .count()
            + 1;

        score *= ((tree_y + 1)..(max_y))
            .take_while(|y| tree > grid[*y][tree_x])
            .count()
            + 1;

        score
    }

    let max_score: usize = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| calc_scenic_score(&grid, (x, y)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    max_score.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 7");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
