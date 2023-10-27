use aoc_client::{PuzzleDay, PuzzleYear};
use clap::Parser;
use std::fmt;
use clap;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum PuzzleEnum {
    P1,
    P2,
}

impl fmt::Display for PuzzleEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PuzzleEnum::P1 => write!(f, "1"),
            PuzzleEnum::P2 => write!(f, "2"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, infer_subcommands = true)]
pub struct Args {
    /// Puzzle day [default: last unlocked day (during Advent of Code month)]
    #[arg(short, long, required = true)]
    pub day: PuzzleDay,
    /// Puzzle year [default: year of current or last Advent of Code event]
    #[arg(short, long, required = true)]
    pub year: PuzzleYear,
    /// Submit answer for the running solution
    #[arg(short, long, value_enum)]
    pub submit: Option<PuzzleEnum>,
}
