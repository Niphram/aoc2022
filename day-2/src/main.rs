use std::str::FromStr;

/// A hand that can be played in rock-paper-scissors
#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

/// A possible outcome of rock-paper-scissors
#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl Hand {
    /// Returns the hand that this hand would win against
    const fn wins_against(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    /// Returns the hand that this hand would loose to
    const fn looses_to(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    /// Plays this hand against the other hand and returns the outcome
    fn play_against(self, other: Self) -> Outcome {
        if self.looses_to() == other {
            Outcome::Loose
        } else if other.looses_to() == self {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }

    /// Returns the hand that would get the specified outcome against the other hand
    const fn from_outcome(other_hand: Self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Loose => other_hand.wins_against(),
            Outcome::Draw => other_hand,
            Outcome::Win => other_hand.looses_to(),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 2");

    let score_part_1: u32 = input
        .lines()
        .map(|l| {
            // Parse both symbols into hands
            let other_hand: Hand = l[..1].parse().unwrap();
            let own_hand: Hand = l[2..].parse().unwrap();

            // Find outcome
            let outcome = own_hand.play_against(other_hand);

            // Calculate score
            outcome as u32 + own_hand as u32
        })
        .sum();

    let score_part_2: u32 = input
        .lines()
        .map(|l| {
            // Parse symbols into hand and outcome
            let other_hand: Hand = l[..1].parse().unwrap();
            let outcome: Outcome = l[2..].parse().unwrap();

            // Find the hand that would achive the outcome
            let own_hand = Hand::from_outcome(other_hand, outcome);

            // Calculate score
            outcome as u32 + own_hand as u32
        })
        .sum();

    println!("Score Part 1: {}", score_part_1);
    println!("Score Part 2: {}", score_part_2);
}
