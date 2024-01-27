pub fn solve(input : &String) {
    part1(&input);
    part2(&input);
}

fn part1(input : &String) {
    let result: i32 = input.chars().map(|c| {
        match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
    }).sum();

    println!("{}", result);
}

fn part2(input : &String) {
    let mut floor = 1;
    for (i, x) in input.chars().enumerate() {
        if floor == 0 {
            println!("{:}", i);
            break;
        }

        floor  = floor + match x {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
    }
}