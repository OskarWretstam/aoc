mod args;

use args::Args;
use clap::{crate_description, crate_name, Parser};
use aoc_client::{AocClient, AocError, AocResult};
use std::process::exit;
use std::path::Path;
use exit_code::*;

fn main() {
    let args = Args::parse();

    println!("{} - {} Year: {:?} Day: {:?} Submit: {:?}",
             crate_name!(),
             crate_description!(),
             args.year,
             args.day,
             args.submit
    );

    let client = match build_client(&args) {
        Ok(client) => client,
        Err(err) => {
            println!("Failed building aoc-client: {err}");
            exit(FAILURE);
        },
    };

    exit(SUCCESS);
}


fn build_client(args: &Args) -> AocResult<AocClient> {
    let mut builder = AocClient::builder();

    builder.session_cookie_from_default_locations()?;

    match (args.year, args.day) {
        (Some(year), Some(day)) => builder.year(year)?.day(day)?,
        (Some(year), None) => builder.year(year)?.latest_puzzle_day()?,
        (None, Some(day)) => builder.latest_event_year()?.day(day)?,
        (None, None) => builder.latest_puzzle_day()?,
    };

    builder
        .input_filename(Path::new("./tmp/input"))
        .puzzle_filename(Path::new("./tmp/puzzle"))
        .build()
}

fn download(client: &AocClient) -> AocResult<()> {
    client.save_puzzle_markdown()?;
    client.save_input()?;
    Ok(())
}
