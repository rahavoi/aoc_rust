use std::cmp;

pub fn solve(input : &String){
    let part1 : i32 = input.split("\n").map(|l| {
        let mut it = l.split("x").into_iter();

        let length = it.next().unwrap().parse::<i32>().unwrap();
        let width = it.next().unwrap().parse::<i32>().unwrap();
        let height = it.next().unwrap().parse::<i32>().unwrap();

        //part_1(&length, &width, &height)
        part_2(&length, &width, &height)
    }).sum();

    println!("{}", part1);
}

fn part_1(length: &i32, width: &i32, height: &i32) -> i32 {
    let paper_needed = 2 * length * width + 2 * length * height + 2 * height * width;
    let slack = cmp::min(cmp::min(length * width, length * height), width * height);
    paper_needed + slack
}

fn part_2(length: &i32, width: &i32, height: &i32) -> i32 {
    let smallest_perimeter = cmp::min(2 * height + 2 * width, cmp::min(2 * length + 2 * width, 2 * length + 2 * height));
    let bow = width * height * length;

    smallest_perimeter + bow

}