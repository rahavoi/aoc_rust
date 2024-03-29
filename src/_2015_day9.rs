use std::cmp;
use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input : &String) {
    let mut min_distance =  i32::MAX;
    let distances = init_map(input);

    let mut cities : Vec<&String> = distances.keys().collect();
    let a_city = cities.remove(0);
    let remaining_cities = cities.len();

    for path in cities.into_iter().permutations(remaining_cities) {
        let mut total_distance = 0;
        let mut max_distance = i32::MIN;

        let mut it = path.iter().peekable();

        let first_city = it.peek().unwrap() as &String;

        total_distance = total_distance + distances.get(a_city).unwrap().get(first_city).unwrap();

        while let Some(city) = it.next() {
            if it.peek().is_some() {
                let next = it.peek().unwrap() as &String;

                let cur_distance = distances
                    .get(*city).unwrap()
                    .get(next).unwrap();

                max_distance = cmp::max(max_distance, cur_distance.clone());
                total_distance = total_distance + cur_distance;
            } else {
                let cur_distance = distances.get(*city).unwrap().get(a_city).unwrap();
                max_distance = cmp::max(max_distance, cur_distance.clone());
                total_distance = total_distance + cur_distance;
            }
        }

        total_distance = total_distance - max_distance;
        min_distance = cmp::min(total_distance, min_distance);
    }

    println!("{}", min_distance);
}

fn init_map(input: &String) -> HashMap<String, HashMap<String, i32>> {
    let mut distances: HashMap<String, HashMap<String, i32>> = HashMap::new();

    input.split("\n").for_each(|l| {
        let mut parts = l.split(" = ");
        let mut cities = parts.next().unwrap().split(" to ");
        let distance = parts.next().unwrap().parse::<i32>().unwrap();
        let from = cities.next().unwrap();
        let to = cities.next().unwrap();

        if distances.get(from).is_none() {
            distances.insert(from.to_string(), HashMap::new());
        }

        if distances.get(to).is_none() {
            distances.insert(to.to_string(), HashMap::new());
        }

        distances.get_mut(from)
            .unwrap()
            .insert(to.to_string(), distance.clone());

        distances.get_mut(to)
            .unwrap()
            .insert(from.to_string(), distance.clone());
    });

    distances
}