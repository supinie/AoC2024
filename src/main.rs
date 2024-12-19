#![feature(let_chains)]

mod day_1;
mod day_2;
mod day_3;
mod day_5;
mod util;

use anyhow::anyhow;
use clap::Parser;

/// Rust Advent of Code 2024 solver. Please add your session token to `session` before using.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Comma seperated list of days to run
    /// [example: 1,2,6]
    #[arg(short, long, value_delimiter = ',', default_value = "1")]
    days: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    for day in args.days {
        let answer = match day {
            1 => day_1::answer().await,
            2 => day_2::answer().await,
            3 => day_3::answer().await,
            5 => day_5::answer().await,
            _ => Err(anyhow!("Invalid day {day}. Either this day has not been solved yet or is not a day in advent.")),
        };
        match answer {
            Ok(number) => println!("day {day}: {:?}", number),
            Err(e) => println!("{e}"),
        }
    }
    Ok(())
}
