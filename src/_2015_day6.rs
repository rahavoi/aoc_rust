use std::cmp;
use std::collections::HashMap;

pub fn solve(input : &str){
    let mut lights = HashMap::new();
    input.split("\n").for_each(|line| {
        let mut parts = line.split(" through ");
        let p1 = parts.next().unwrap();
        let p2 = parts.next().unwrap();

        let operation;
        let part1;

        if p1.starts_with("turn on ") {
            operation = Operation::TurnOn;
            part1 = p1.replace("turn on ", "").clone();
        } else if p1.starts_with("turn off ") {
            operation = Operation::TurnOff;
            part1 = p1.replace("turn off ", "").clone();
        } else {
            operation = Operation::Toggle;
            part1 = p1.replace("toggle ", "").clone();
        }

        let mut  from_parts = part1.split(",");
        let mut to_parts = p2.split(",");

        let from_x = from_parts.next().unwrap().parse::<i32>().unwrap();
        let from_y = from_parts.next().unwrap().parse::<i32>().unwrap();
        let to_x = to_parts.next().unwrap().parse::<i32>().unwrap() + 1;
        let to_y = to_parts.next().unwrap().parse::<i32>().unwrap() + 1;

        for x in from_x..to_x {
            for y in from_y..to_y {
                let key = format!("{},{}", x, y);
                match operation {
                    //part one:
                    /*
                    Operation::TurnOn => {lights.insert(key, 1);},
                    Operation::TurnOff => {lights.insert(key, 0);},
                    Operation::Toggle => {
                        let cur = lights.get(&key).get_or_insert(&0).clone();

                        if cur == 0 {
                            lights.insert(key, 1);
                        } else {
                            lights.insert(key, 0);
                        }
                    },
                     */
                    //part two:
                    Operation::TurnOn => {lights.entry(key).and_modify(|v| *v += 1).or_insert(1)},
                    Operation::TurnOff => {lights.entry(key).and_modify(|v| {
                        *v = cmp::max(&*v - 1, 0);
                    }).or_insert(0)},
                    Operation::Toggle => {lights.entry(key).and_modify(|v| *v += 2).or_insert(2)},
                };
            }
        }
    });
    println!("{}", lights.values().sum::<i32>());
}

enum Operation {
    TurnOn,
    TurnOff,
    Toggle
}
