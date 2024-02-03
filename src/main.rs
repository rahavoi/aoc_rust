use std::fs;
use std::time::Instant;

mod _2015_day10;

fn main() {
    let input = parse_input();
    let now = Instant::now();
    //_2015_day1::solve(&input);
    //_2015_day2::solve(&input);
    //_2015_day3::solve(&input);
    //_2015_day4::solve("ckczppom");
    //_2015_day5::solve(&input);
    //_2015_day6::solve(&input);
    //_2015_day7::solve(&input);
    //_2015_day8::solve(&input);
    _2015_day10::solve("3113322113");

   // _2015_day9::solve(&input);

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn parse_input() -> String{
    let path = std::path::Path::new("/Users/irahavoi/IdeaProjects/aoc_rust/resources/2015/Day8.txt");
    fs::read_to_string(path).expect("No Such File")
}
