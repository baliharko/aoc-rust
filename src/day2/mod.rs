use crate::util::get_input;
use std::collections::HashMap;

fn points(p1: usize, p2: usize) -> usize {
    return if (p1 == p2 + 1) || (p1 == 1 && p2 == 3) {
        p2
    } else if p1 == p2 {
        3 + p2
    } else if (p2 == p1 + 1) || (p1 == 3 && p2 == 1) {
        6 + p2
    } else {
        0
    };
}

pub fn part_1() -> usize {
    let input: String = get_input("inputs/day2/input.txt");

    let shapes: HashMap<char, usize> = [('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]
        .iter()
        .cloned()
        .collect();

    let out: usize = input
        .split("\n")
        .map(|round| {
            let r: Vec<usize> = round
                .split(' ')
                .map(|shape| -> usize {
                    let key: char = shape.chars().nth(0).unwrap();
                    return shapes[&key];
                })
                .collect();

            let opponent = r[0];
            let player = r[1];
            return points(opponent, player);
        })
        .sum();

    return out;
}
