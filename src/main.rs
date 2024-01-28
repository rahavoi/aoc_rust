use std::fs;
use std::time::Instant;

mod _2015_day9;

fn main() {
    let input = parse_input();
    let now = Instant::now();
    //_2015_day1::solve(&input);
    //_2015_day2::solve(&input);
    //_2015_day3::solve(&input);


    _2015_day9::solve(&input);

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn parse_input() -> String{
    let path = std::path::Path::new("/Users/irahavoi/IdeaProjects/aoc_rust/resources/2015/Day9.txt");
    fs::read_to_string(path).expect("No Such File")
}
