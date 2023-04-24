const INPUT: &str = include_str!("../../inputs/day5/input.txt");

pub fn part_1() -> String {
    let (stacks, instructions) = input_as_stacks_and_instructions();
    let moved_stacks = do_move_p1(&stacks, &instructions);
    return get_top_crates(&moved_stacks);
}

pub fn part_2() -> String {
    let (stacks, instructions) = input_as_stacks_and_instructions();
    let moved_stacks = do_move_p2(&stacks, &instructions);
    return get_top_crates(&moved_stacks);
}

fn do_move_p1(stacks: &Vec<Vec<char>>, instructions: &Vec<Vec<usize>>) -> Vec<Vec<char>> {
    instructions.iter().fold(stacks.clone(), |mut acc, it| {
        let moves: usize = it[0];
        let from: usize = it[1] - 1;
        let to: usize = it[2] - 1;

        (0..moves).for_each(|_| {
            let temp = acc[from].pop().unwrap();
            acc[to].push(temp);
        });

        return acc
    })
}

fn do_move_p2(stacks: &Vec<Vec<char>>, instructions: &Vec<Vec<usize>>) -> Vec<Vec<char>> {
    instructions.iter().fold(stacks.clone(), |mut acc, it| {
        let amount: usize = it[0];
        let from: usize = it[1] - 1;
        let to: usize = it[2] - 1;

        let split_index = &acc[from].len() - amount;
        let temp = acc[from].split_off(split_index);

        acc[to].extend(temp.iter());
        return acc
    })
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .fold(String::new(), |mut acc, it| -> String {
            acc.push(it.last().cloned().unwrap());
            return acc
        })
}

fn input_as_stacks_and_instructions() -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let input: Vec<&str> = INPUT.split("\n\n").collect();
    let stacks: Vec<Vec<char>> = input_to_stacks(input[0]);
    let instructions: Vec<Vec<usize>> = input_to_instructions(input[1].clone());

    return (stacks, instructions);
}

fn input_to_instructions(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| {
        line
            .split(" ")
            .filter_map(|it| it.parse::<usize>().ok()).collect::<Vec<usize>>()
    })
    .collect::<Vec<Vec<usize>>>()
}

fn input_to_stacks(input: &str) -> Vec<Vec<char>> {
    let stacks_amount: usize = get_stacks_amount(input);
    let chars_per_stack: usize = (input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .len() + 1) / stacks_amount;

    let stack_height = &input.lines().count() - 1;
    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(stack_height); stacks_amount];

    input
        .lines()
        .rev()
        .skip(1)
        .for_each(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
                .chunks(chars_per_stack)
                .enumerate()
                .for_each(|(col, krate)| {

                    let contents = krate[1];
                    if contents != ' ' {
                        stacks[col].push(contents);
                    }
                })
        });

    return stacks;
}

fn get_stacks_amount(stacks: &str) -> usize {
    return stacks
        .lines()
        .last()
        .unwrap()
        .replace(" ", "")
        .chars()
        .map(|it| it.to_digit(10).unwrap_or(0) as usize)
        .max()
        .unwrap();
}
