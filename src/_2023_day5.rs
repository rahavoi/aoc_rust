pub fn solve(input : &str){
    let mut lines = input.split("\n");

    let seeds_input = lines.next().unwrap().replace("seeds: ", "");
    let seeds_part1 : Vec<i64> = seeds_input.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();

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
}

fn part1(seeds: &Vec<i64>, almanac: &Vec<Vec<Rule>>) -> i64 {
    let mut locations = Vec::new();

    for seed in seeds {
        let mut source = seed.to_owned();

        for rules in almanac {
            source = get_destination_from_source(&source, &rules);
        }

        locations.push(source);
    }

    locations.iter().min().unwrap().to_owned()
}

fn get_destination_from_source(source : &i64, rules : &Vec<Rule>) -> i64 {
    for r in rules {
        if source >= &r.source_idx && source < &(&r.source_idx + &r.length) {
            return &r.destination_idx + source - &r.source_idx;
        }
    }

    return source.clone()   ;
}

struct Rule {
    source_idx : i64,
    destination_idx : i64,
    length : i64
}