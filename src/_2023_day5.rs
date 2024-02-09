use std::cmp;
use std::collections::vec_deque::VecDeque;

pub fn solve(input : &str){
    let mut lines = input.split("\n");

    let seeds_input = lines.next().unwrap().replace("seeds: ", "");
    let seeds_part1 : Vec<i64> = seeds_input.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
    let ranges_part2 : VecDeque<Range> = parse_ranges(&seeds_input);

    lines.next();
    lines.next();

    let mut almanac : Vec<Vec<Rule>>  = Vec::new();
    let mut cur : Vec<Rule> = Vec::new();

    while let Some(s) = lines.next() {
        if s.is_empty() {
            almanac.push(cur);
            cur = Vec::new();
            lines.next();
        } else {
            let mut parts = s.split(" ");
            cur.push(Rule{
                destination_idx : parts.next().unwrap().parse::<i64>().unwrap(),
                source_idx : parts.next().unwrap().parse::<i64>().unwrap(),
                length : parts.next().unwrap().parse::<i64>().unwrap(),
            })
        }
    }

    almanac.push(cur);

    println!("Part1: {}", part1(&seeds_part1, &mut almanac));
    println!("Part1: {}", part2(ranges_part2, &mut almanac));
}

fn part1(seeds: &Vec<i64>, almanac: &Vec<Vec<Rule>>) -> i64 {
    let mut locations = Vec::new();

    for seed in seeds {
        locations.push(get_location_for_seed(seed.to_owned(), almanac));
    }

    locations.iter().min().unwrap().to_owned()
}

fn part2(mut ranges: VecDeque<Range>, almanac: &Vec<Vec<Rule>>) -> i64{
    let mut min_location = i64::MAX;

    while !ranges.is_empty() {
        //println!("Ranges to check: {}", ranges.len());
        let range = ranges.pop_front().unwrap();
        let start = range.start.to_owned();
        let length = range.length;
        let end = range.start + length.to_owned() - 1;

        if start >= end {
            continue;
        }

        let start_seed_location = get_location_for_seed(start, almanac);
        let end_seed_location = get_location_for_seed(end, almanac);

        if start_seed_location == end_seed_location - length + 1 {
            //Found perfect continous range, no need to check all elements
            min_location = cmp::min(min_location, start_seed_location);
        } else {
            if length.to_owned() > 2 {
                ranges.push_back(Range {
                    start : start.to_owned(),
                    length : length.to_owned() / 2
                });

                ranges.push_back(Range {
                    start : start.to_owned() + length.to_owned() / 2  + 1,
                    length : length.to_owned() / 2
                });
            }
        }
    }

    min_location
}

fn get_location_for_seed(seed : i64, almanac: &Vec<Vec<Rule>>) -> i64 {
    let mut source = seed.to_owned();

    for rules in almanac {
        source = get_destination_from_source(&source, &rules);
    }

    source
}

fn get_destination_from_source(source : &i64, rules : &Vec<Rule>) -> i64 {
    for r in rules {
        if source >= &r.source_idx && source < &(&r.source_idx + &r.length) {
            return &r.destination_idx + source - &r.source_idx;
        }
    }

    return source.clone()   ;
}

fn parse_ranges(input : &str) -> VecDeque<Range> {
    let mut ranges = VecDeque::new();
    let mut it = input.split(" ");
    while let Some(s) = it.next() {
        ranges.push_back(Range {
            start : s.parse::<i64>().expect("yolo"),
            length : it.next().unwrap().parse::<i64>().expect("yolo"),
        })
    }

    ranges
}

struct Rule {
    source_idx: i64,
    destination_idx: i64,
    length: i64
}

struct Range {
    start : i64,
    length : i64,
}