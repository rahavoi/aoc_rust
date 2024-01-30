use std::collections::HashSet;

pub fn solve(input : &str){
    part1(input);
    part2(input);
}

fn part2(input : &str) {
    let nice_count = input.split("\n")
        .filter(|l| contains_repeating_letters_with_exactly_one_letter_between(l))
        .filter(|l| contains_pair_without_ovelapping(l))
        .count();

    println!("{}", nice_count);
}

fn contains_repeating_letters_with_exactly_one_letter_between(input: &str) -> bool{
    for i in 2..input.len() {
        if input.chars().nth(&i - 2) == input.chars().nth(i) {
            return true
        }
    }

    return false
}

fn contains_pair_without_ovelapping(input: &str) -> bool{
    let mut combinations = HashSet::new();
    let mut prev = format!("{}{}", input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap());

    for i in 2..input.len() {
        let cur = format!("{}{}", input.chars().nth(i - 1).unwrap(), input.chars().nth(i).unwrap());

        if combinations.contains(&cur) {
            return true;
        } else {
            combinations.insert(prev.clone());
            prev = cur;
        }
    }

    return false;
}

fn part1(input: &str) {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let except = vec!["ab", "cd", "pq", "xy"];

    let nice_count = input.split("\n")
        .filter(|l| contains_at_least_one_letter_that_appears_twice_in_a_row(l))
        .filter(|l| contains_at_least_3_vowels(&vowels, l))
        .filter(|l| !except.iter().any(|word| l.contains(word)))
        .count();

    println!("{}", nice_count);
}

fn contains_at_least_one_letter_that_appears_twice_in_a_row(line : &str) -> bool{
    let mut chars = line.chars();
    let mut prev = chars.next().unwrap();

    for c in chars {
        if c == prev {
            return true
        } else {
            prev = c;
        }
    }

    false
}

fn contains_at_least_3_vowels(vowels: &Vec<char>, l: &str) -> bool {
    let mut count_vowels = 0;

    for c in l.chars() {
        if vowels.contains(&c) {
            count_vowels = count_vowels + 1;
        }

        if count_vowels == 3 {
            return true;
        }
    }

    false
}