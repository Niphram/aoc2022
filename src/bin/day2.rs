use std::str::FromStr;

/// A hand that can be played in rock-paper-scissors
#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

/// A possible outcome of rock-paper-scissors
#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    LOOSE = 0,
    DRAW = 3,
    WIN = 6,
}

impl Hand {
    /// Returns the hand that this hand would win against
    fn wins_against(&self) -> Self {
        match self {
            Self::ROCK => Self::SCISSORS,
            Self::PAPER => Self::ROCK,
            Self::SCISSORS => Self::PAPER,
        }
    }

    /// Returns the hand that this hand would loose to
    fn looses_to(&self) -> Self {
        match self {
            Self::ROCK => Self::PAPER,
            Self::PAPER => Self::SCISSORS,
            Self::SCISSORS => Self::ROCK,
        }
    }

    /// Plays this hand against the other hand and returns the outcome
    fn play_against(&self, other: &Hand) -> Outcome {
        if self.looses_to() == *other {
            Outcome::LOOSE
        } else if other.looses_to() == *self {
            Outcome::WIN
        } else {
            Outcome::DRAW
        }
    }

    /// Returns the hand that would get the specified outcome against the other hand
    fn from_outcome(other_hand: &Hand, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::LOOSE => other_hand.wins_against(),
            Outcome::DRAW => *other_hand,
            Outcome::WIN => other_hand.looses_to(),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::ROCK),
            "B" | "Y" => Ok(Self::PAPER),
            "C" | "Z" => Ok(Self::SCISSORS),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::LOOSE),
            "Y" => Ok(Self::DRAW),
            "Z" => Ok(Self::WIN),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = include_str!("inputs/day2.txt");

    println!("Advent of Code 2022 - Day 2");

    let score_part_1: u32 = input
        .split("\n")
        .map(|l| {
            // Parse both symbols into hands
            let other_hand: Hand = l[..1].parse().unwrap();
            let own_hand: Hand = l[2..].parse().unwrap();

            // Find outcome
            let outcome = own_hand.play_against(&other_hand);

            // Calculate score
            outcome as u32 + own_hand as u32
        })
        .sum();

    let score_part_2: u32 = input
        .split("\n")
        .map(|l| {
            // Parse symbols into hand and outcome
            let other_hand: Hand = l[..1].parse().unwrap();
            let outcome: Outcome = l[2..].parse().unwrap();

            // Find the hand that would achive the outcome
            let own_hand = Hand::from_outcome(&other_hand, &outcome);

            // Calculate score
            outcome as u32 + own_hand as u32
        })
        .sum();

    println!("Score Part 1: {}", score_part_1);
    println!("Score Part 2: {}", score_part_2);
}
