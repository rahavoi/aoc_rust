use std::cmp;
use std::collections::HashMap;

use itertools::Itertools;

pub struct DistancesData {
    distances : HashMap<String, HashMap<String, i32>>,
    longest_from: String,
    longest_to: String
}

pub fn solve(input : &String) {
    let mut min_distance =  i32::MAX;
    let distances_data = init_map(input);
    let distances = distances_data.distances;

    for path in distances.keys().into_iter().permutations(distances.len()) {
        //println!("{:?}", path);
        let mut distance = 0;

        let mut it = path.iter().peekable();

        let mut copy = it.clone();


        let mut avoid_longest_commute = true;

        //Trick here is to completely avoid paths containing the longest commute.
        //This way we reduce the number of permutations we need to check.
        //Assumption: checking if a path contains a pair of cities with longest commute
        //is cheaper than calculating the total travelling distance of the path.
        while let Some(city) = copy.next() {
            if copy.peek().is_some() {
                let next = copy.peek().unwrap() as &String;

                if(*city == &distances_data.longest_to || *city == &distances_data.longest_from) &&
                    (next == &distances_data.longest_to || next == &distances_data.longest_from) {
                    avoid_longest_commute = false;
                    break
                }
            }
        }

        if !avoid_longest_commute {
            continue;
        }

        while let Some(city) = it.next() {
            if it.peek().is_some() {
                let next = it.peek().unwrap() as &String;
                distance = distance + distances
                    .get(*city).unwrap()
                    .get(next).unwrap();
            }
        }

        min_distance = cmp::min(distance, min_distance);
    }

    println!("{}", min_distance);
}

fn init_map(input: &String) -> DistancesData {
    let mut distances: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut longest_distance = i32::MIN;
    let mut longest_from= "";
    let mut longest_to= "";


    input.split("\n").for_each(|l| {
        let mut parts = l.split(" = ");
        let mut cities = parts.next().unwrap().split(" to ");
        let distance = parts.next().unwrap().parse::<i32>().unwrap();

        let from = cities.next().unwrap();
        let to = cities.next().unwrap();

        if distance > longest_distance {
            longest_distance = distance;
            longest_to = to;
            longest_from = from;

        }

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

    DistancesData {
        distances,
        longest_from : longest_from.to_string(),
        longest_to : longest_to.to_string()
    }
}