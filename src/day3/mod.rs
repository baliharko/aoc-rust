use std::collections::HashSet;
use crate::util::get_input;

const INPUT_PATH: &str = "inputs/day3/input.txt";

fn get_priority(item: char) -> isize {
    let char_val = item as isize + 1;

    return if char_val > 91 {
        char_val - 'a' as isize
    } else {
        char_val - 'A' as isize + 26
    };
}

pub fn part_1() -> isize {
    let input = get_input(INPUT_PATH);
    let rucksacks: Vec<&str> = input.split("\n").collect();

    let priorities_sum: isize = rucksacks
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first_unique: HashSet<char> = HashSet::from_iter(first.chars());
            let second_unique: HashSet<char> = HashSet::from_iter(second.chars());

            return first_unique
                .into_iter()
                .filter(move|it| second_unique.contains(it));
        })
        .flatten()
        .map(|it| get_priority(it))
        .sum();

    return priorities_sum;
}

pub fn part_2() -> isize {
    let input = get_input(INPUT_PATH);
    let rucksacks: Vec<String> = input.split("\n").map(|it| it.to_string()).collect();

    let priorities_sum: isize = rucksacks
        .chunks(3)
        .map(|chunk| -> Vec<char> {
            let a: HashSet<char> = HashSet::from_iter(chunk[0].chars());
            let b: HashSet<char> = HashSet::from_iter(chunk[1].chars());
            let c: HashSet<char> = HashSet::from_iter(chunk[2].chars());

            // println!("\n{:?}\n{:?}\n{:?}\n", a, b, c);

            let badge: Vec<char> = a.iter()
                .cloned()
                .filter(|it| b.contains(it) && c.contains(it))                
                .collect();

            return badge;
        })
        .flatten()
        .map(|it| get_priority(it))
        .sum();

    return priorities_sum;
}
