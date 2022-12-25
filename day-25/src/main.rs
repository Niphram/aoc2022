use std::{fmt::Display, iter::Sum, ops::Add, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Snafu(isize);

impl Add for Snafu {
    type Output = Self;

    /// Adds two snafus
    fn add(self, rhs: Self) -> Self::Output {
        Snafu(self.0 + rhs.0)
    }
}

impl Sum for Snafu {
    /// Sums an iterator of snafus
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Snafu(0), |a, b| a + b)
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = 0;

        // Go through all characters
        for (i, c) in s.chars().rev().enumerate() {
            // Match digits
            let digit = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => Err(())?,
            };

            let i = i.try_into().map_err(|_| ())?;

            // Sum digits
            res += digit * (5isize.pow(i));
        }

        Ok(Snafu(res))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            // If the number is 0 just write that
            write!(f, "0")
        } else {
            let positive = self.0 >= 0;
            let mut number = self.0.unsigned_abs();

            let mut result = vec![];

            // Divide number by 5 until we reach 0
            while number != 0 {
                let remainder = number % 5;
                number /= 5;

                // snafu numbers use -2 to 2 instead of 0 to 5, so we carry if needed
                if remainder > 2 {
                    number += 1;
                }

                // Push remainder to result
                result.push(remainder);
            }

            // Lookup
            let lookup = if positive {
                ['0', '1', '2', '=', '-']
            } else {
                ['0', '-', '=', '2', '1']
            };

            for digit in result.iter().rev() {
                write!(f, "{}", lookup[*digit])?;
            }

            Ok(())
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 25");

    let fuel_sum: Snafu = input.lines().flat_map(&str::parse::<Snafu>).sum();
    println!("Part 1: {fuel_sum}");
}
