use aoc2022::day1;
use clap::Parser;

#[derive(clap::Parser)]
struct Args{
    #[arg(short,long,value_name = "Day",default_value_t = 1)]
    day: u8
}

fn main() {
    let input = Args::parse();
    match input.day {
        1=>day1::solve(),
        _=> println!("noch nicht fertig"),
    }
}
