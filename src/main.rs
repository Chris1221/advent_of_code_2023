//! # Advent of code 2023

mod days;
use days::day1;
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
}

fn main() {
    let opt = Opt::from_args();

    match opt.day {
        1 => day1::day1(opt.challenge),
        _ => println!("Day {} not implemented", opt.day),
    }
}
