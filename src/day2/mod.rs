use crate::util::get_input;
use std::collections::HashMap;

const INPUT_PATH: &str = "inputs/day2/input.txt";

fn points(p1: usize, p2: usize) -> usize {
    return if p1 == (p2 % 3) + 1 {
        p2
    } else if p1 == p2 {
        3 + p2
    } else if p2 == (p1 % 3) + 1 {
        6 + p2
    } else {
        0
    };
}

fn shapes() -> HashMap<char, usize> {
    return [('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]
        .iter()
        .cloned()
        .collect();
}

pub fn part_1() -> usize {
    let input: String = get_input(INPUT_PATH);
    return input
        .split("\n")
        .map(|round| {
            let r: Vec<usize> = round
                .split(' ')
                .map(|shape| -> usize {
                    let key: char = shape.chars().nth(0).unwrap();
                    return shapes()[&key];
                })
                .collect();

            let opponent = r[0];
            let player = r[1];

            return points(opponent, player);
        })
        .sum();
}

pub fn part_2() -> usize {
    let input: String = get_input(INPUT_PATH);

    fn necessary_move(p1: usize, target_outcome: usize) -> (usize, usize) {
        let necessary_move;
        match target_outcome {
            1 => necessary_move = ((p1 + 1) % 3) + 1,
            2 => necessary_move = p1,
            3 => necessary_move = (p1 % 3) + 1,
            _ => {
                panic!("{p1} is not a valid move!")
            }
        }
                
        return (p1, necessary_move);
    }

    return input
        .split("\n")
        .map(|round| -> usize {
            let r: Vec<usize> = round
                .split(' ')
                .map(|shape| -> usize {
                    let key: char = shape.chars().nth(0).unwrap();
                    return shapes()[&key];
                })
                .collect();

            let (opponent, player) = necessary_move(r[0], r[1]);
            let points: usize = points(opponent, player);

            // println!("Round: {round} - op: {opponent}, p: {player} -> {points}");
            return points;
        })
        .sum();
}
