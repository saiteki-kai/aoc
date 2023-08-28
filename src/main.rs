use clap::Parser;

mod days;

#[derive(Parser, Default, Debug)]
#[clap(name = "Advent of Code Solutions", version = "1.0")]
struct Arguments {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value="1")]
    solution_version: Option<u8>,
}

fn main() {
    let args = Arguments::parse();

    let v = args.solution_version;

    match args.day {
        1 => {days::day01::part1::solve(v); days::day01::part2::solve()},
        _ => println!("Please specify a number between 1 and 25."),
    }
}
