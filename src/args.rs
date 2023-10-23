use aoc_client::{PuzzleDay, PuzzleYear};
use clap::Parser;
use clap;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum PuzzlePart {
    P1,
    P2,
}
#[derive(Parser, Debug)]
#[command(version, about, infer_subcommands = true)]
pub struct Args {
    /// Puzzle day [default: last unlocked day (during Advent of Code month)]
    #[arg(short, long, required = true)]
    pub day: Option<PuzzleDay>,
    /// Puzzle year [default: year of current or last Advent of Code event]
    #[arg(short, long, required = true)]
    pub year: Option<PuzzleYear>,
    /// Submit answer for the running solution
    #[arg(short, long, value_enum)]
    pub submit: Option<PuzzlePart>,
}
