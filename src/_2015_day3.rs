use std::collections::HashMap;

pub fn solve(input : &String){
    part1(input);
    part2(input);
}

fn part1(input: &String) {
    let mut pos_count = HashMap::new();
    pos_count.insert("0,0".to_string(), 1);

    let mut x = 0;
    let mut y = 0;

    input.chars().for_each(|c| {
        match c {
            '>' => x = x + 1,
            '<' => x = x - 1,
            '^' => y = y + 1,
            'v' => y = y - 1,
            _ => panic!("Wrong instruction: {}", c)
        };

        let key = format!("{},{}", &x, &y);

        *pos_count.entry(key.to_string()).or_insert(0) += 1;
    });

    println!("{}", pos_count.keys().len())
}

fn part2(input: &String) {
    let mut pos_count = HashMap::new();
    pos_count.insert("0,0".to_string(), 2);

    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robot_x = 0;
    let mut robot_y = 0;

    let mut santas_turn = true;

    input.chars().for_each(|c| {
        let mut x = if santas_turn { santa_x } else { robot_x };
        let mut y = if santas_turn { santa_y } else { robot_y };

        match c {
            '>' => x = x + 1,
            '<' => x = x - 1,
            '^' => y = y + 1,
            'v' => y = y - 1,
            _ => panic!("Wrong instruction: {}", c)
        };

        let key = format!("{},{}", &x, &y);

        *pos_count.entry(key.to_string()).or_insert(0) += 1;

        if santas_turn {
            santa_x = x;
            santa_y = y;
        } else {
            robot_x = x;
            robot_y = y;
        }

        santas_turn = !santas_turn;
    });

    println!("{}", pos_count.keys().len())
}