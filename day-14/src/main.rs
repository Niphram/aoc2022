type Pos = (isize, isize);

#[derive(Debug)]
struct Bounds {
    top: isize,
    right: isize,
    bottom: isize,
    left: isize,
}

impl Bounds {
    fn from_pos(pos: &Pos) -> Self {
        Bounds {
            top: pos.1,
            right: pos.0,
            bottom: pos.1,
            left: pos.0,
        }
    }

    fn width(&self) -> usize {
        isize::abs_diff(self.left, self.right) + 1
    }

    fn height(&self) -> usize {
        isize::abs_diff(self.top, self.bottom) + 1
    }

    fn expand(&mut self, pos: &Pos) {
        self.top = pos.1.min(self.top);
        self.bottom = pos.1.max(self.bottom);
        self.left = pos.0.min(self.left);
        self.right = pos.0.max(self.right);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
}

struct Simulation {
    grid: Vec<Vec<Material>>,
    origin: Pos,
}

impl Simulation {
    fn new(bounds: &Bounds) -> Self {
        // Generate grid
        let grid = (0..bounds.height())
            .map(|_| [Material::Air].repeat(bounds.width()))
            .collect();

        let origin = (bounds.left, bounds.top);

        Self { grid, origin }
    }

    /// Draw a line of rocks into the grid
    fn draw_line(&mut self, line: &[Pos]) {
        for points in line.windows(2) {
            let mut start = points[0];
            let end = points[1];

            while start != end {
                self.set(&start, Material::Rock);

                start.0 += (end.0 - start.0).signum();
                start.1 += (end.1 - start.1).signum();
            }
        }

        self.set(line.last().unwrap(), Material::Rock);
    }

    /// Set position in the grid
    fn set(&mut self, pos: &Pos, mat: Material) {
        self.grid[(pos.1 - self.origin.1) as usize][(pos.0 - self.origin.0) as usize] = mat;
    }

    /// Check if position is air
    fn is_free(&self, pos: &Pos) -> Option<bool> {
        self.grid
            .get(usize::try_from(pos.1 - self.origin.1).ok()?)?
            .get(usize::try_from(pos.0 - self.origin.0).ok()?)
            .map(|&mat| mat == Material::Air)
    }

    /// Simulate sand falling until it can't move anymore or leaves the grid
    fn simulate_sand(&mut self, pos: &Pos) -> Option<Pos> {
        let mut pos = *pos;

        loop {
            if self.is_free(&(pos.0, pos.1 + 1))? {
                // Down
                pos = (pos.0, pos.1 + 1);
            } else if self.is_free(&(pos.0 - 1, pos.1 + 1))? {
                // Down-left
                pos = (pos.0 - 1, pos.1 + 1);
            } else if self.is_free(&(pos.0 + 1, pos.1 + 1))? {
                // Down-right
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                // Can't move anymore
                self.set(&pos, Material::Sand);
                break;
            }
        }

        Some(pos)
    }
}

/// Parse input into a grid and calculate bounds
fn parse_lines(input: &str) -> (Vec<Vec<Pos>>, Bounds) {
    let lines: Vec<Vec<Pos>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();

                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    // Find bounds
    let mut bounds = Bounds::from_pos(&(500, 0));
    lines
        .iter()
        .for_each(|points| points.iter().for_each(|pos| bounds.expand(pos)));

    (lines, bounds)
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let (lines, bounds) = parse_lines(input);

    let mut sim = Simulation::new(&bounds);

    // Draw all lines into simulation
    for line in lines {
        sim.draw_line(&line);
    }

    // Simulate until sand doesn't rest
    let mut count = 0;
    while sim.simulate_sand(&(500, 0)).is_some() {
        count += 1;
    }

    count.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let (lines, mut bounds) = parse_lines(input);

    // Expand bottom by 2
    bounds.bottom += 2;

    // Change bounds to make sure no sand falls off
    let height = bounds.height();
    bounds.left = isize::min(bounds.left, 500 - (height as isize));
    bounds.right = isize::max(bounds.right, 500 + (height as isize));

    let mut sim = Simulation::new(&bounds);

    // Draw all lines into simulation
    for line in lines {
        sim.draw_line(&line);
    }

    // Add ground
    sim.draw_line(&[(bounds.left, bounds.bottom), (bounds.right, bounds.bottom)]);

    // Simulate until sand comes to rest at (500, 0) or falls off
    let mut count = 0;
    while let Some(pos) = sim.simulate_sand(&(500, 0)) {
        count += 1;

        if pos == (500, 0) {
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
