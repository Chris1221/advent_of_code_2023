//! # Advent of code 2023

mod days;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "aoc23", about = "Advent of code 2023")]
struct Opt {
    /// Day to run
    #[structopt(short, long)]
    day: u8,

    /// Challenge to run
    #[structopt(short, long)]
    challenge: u8,

    /// Input file
    #[structopt(short, long)]
    input: String,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();

    match opt.day {
        1 => days::day1::day(opt.challenge, &opt.input),
        2 => days::day2::day(opt.challenge, &opt.input),
        3 => days::day3::day(opt.challenge, &opt.input),
        _ => 0_u32,
    };

    Ok(())
}
