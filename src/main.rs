mod args;
mod unsolved;

use unsolved::AocFunctions;
use args::Args;
use clap::Parser;
use std::io::prelude::*;
use aoc_client::{AocClient, AocResult};
use std::process::exit;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use exit_code::*;


struct AocSolver {
    key: String,
    client: AocClient,
    functions: AocFunctions
}

impl AocSolver {

    fn build_client(year: i32, day: u32, path: &PathBuf) -> AocResult<AocClient> {
        let mut input_path = path.clone();
        let mut puzzle_path = path.clone();
        input_path.push("input");
        puzzle_path.push("puzzle");

        AocClient::builder()
            .session_cookie_from_default_locations()?
            .year(year)?
            .day(day)?
            .input_filename(input_path)
            .puzzle_filename(puzzle_path)
            .overwrite_files(true)
            .build()
    }

    fn build(args: &Args) -> Option<AocSolver> {
        let mut path = PathBuf::from(format!("./src/y{}/d{}", args.year, args.day));

        let client = match Self::build_client(args.year, args.day, &path) {
            Ok(client) => client,
            Err(err) => {
                println!("Failed to build AocClient, error {err}");
                return None;
            },
        };

        if !path.exists() {
            if let Err(err) = fs::create_dir_all(&path) {
                println!("Failed to create data directory, error {err}");
                return None;
            }

            path.push("solution.rs");
            let initial_file = format!("pub fn s{}{}() -> String {{ String::from(\"Not implemented\")}}", args.year, args.day);
            if let Err(err) = File::create(path).and_then(
                |mut file| file.write_all(initial_file.as_bytes())
            ) {
                println!("Failed to create solution file, error {err}");
                return None;
            }

            if let Err(err) = client.save_puzzle_markdown() {
                println!("Failed to save puzzle data, error {err}");
                return None;
            }

            if let Err(err) = client.save_input() {
                println!("Failed to save input data, error {err}");
                return None;
            }
        }

        Some(AocSolver {
            client: client,
            functions: AocFunctions::new(),
            key: format!("{}{}", args.year, args.day)
        })
    }

    fn solve(&self) -> String {
        self.functions.get(&self.key)()
    }
}

fn main() {
    let args = Args::parse();

    let solver =  match AocSolver::build(&args) {
        Some(solver) => solver,
        None => {
            println!("Failed to setup solver, exiting.");
            exit(FAILURE);
        },
    };

    let solution = solver.solve();
    println!("Solution is: {solution}");

    if let Some(submit) = args.submit {
        if solution == "Not implemented" {
            println!("Will not submit the answer \"Not implemented\"");
            exit(FAILURE);
        }

        let to_answer = submit.to_string();
        if let Err(error) = solver.client.submit_answer_and_show_outcome(&to_answer, solution)
        {
            println!("Failed to submit, error {:?}", error);
            exit(FAILURE);
        }
    }


    exit(SUCCESS);
}
