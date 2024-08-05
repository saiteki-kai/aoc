use clap::Parser;

mod days;

#[derive(Parser, Default, Debug)]
#[clap(name = "Advent of Code Solutions", version = "1.0")]
struct Arguments {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value = "1")]
    solution_version: Option<u8>,
}

fn main() {
    let args = Arguments::parse();

    let v = args.solution_version;

    match args.day {
        1 => {
            days::day01::part1::solve(v);
            days::day01::part2::solve()
        }
        2 => {
            days::day02::part1::solve();
            days::day02::part2::solve()
        }
        3 => {
            days::day03::part1::solve();
            days::day03::part2::solve()
        }
        4 => {
            days::day04::part1::solve();
            days::day04::part2::solve()
        }
        5 => {
            days::day05::part1::solve();
            days::day05::part2::solve()
        }
        6 => {
            days::day06::part1::solve();
            days::day06::part2::solve()
        }
        7 => {
            days::day07::part1::solve();
            days::day07::part2::solve()
        }
        8 => {
            days::day08::part1::solve();
            days::day08::part2::solve()
        }
        9 => {
            days::day09::part1::solve();
            days::day09::part2::solve()
        }
        12 => {
            days::day12::part1::solve();
            days::day12::part2::solve();
            days::day12::visualization::main();
        }
        _ => println!("Please specify a number between 1 and 25."),
    }
}
