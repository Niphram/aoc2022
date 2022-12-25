use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Snafu(isize);

impl FromStr for Snafu {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    let digit = match c {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!(),
                    };

                    digit * (5isize.pow(i as u32))
                })
                .sum(),
        ))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = self.0;

        let mut digits = vec![];

        while c != 0 {
            let mut rem = c % 5;

            if rem > 2 {
                c += 5;
                rem -= 5;
            }

            c /= 5;

            digits.push(rem);
        }

        for d in digits.iter().rev() {
            write!(
                f,
                "{}",
                match d {
                    2 => '2',
                    1 => '1',
                    0 => '0',
                    -1 => '-',
                    -2 => '=',
                    _ => panic!(),
                }
            )?;
        }

        Ok(())
    }
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let fuel_sum: isize = input
        .lines()
        .flat_map(&str::parse::<Snafu>)
        .map(|s| s.0)
        .sum();

    Snafu(fuel_sum).to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 25");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");
}
