use crate::util::get_input;

fn elves_by_cals_sorted() -> Vec<u64> {
    let input: &str = &get_input("inputs/day1/input.txt");

    let elf_cals = input
        .split("\n\n")
        .map(|elf| -> u64 {
            elf.split("\n")
                .map(|item| -> u64 { item.parse().unwrap_or(0) })
                .sum()
        })
        .collect::<Vec<_>>();

    let mut sorted = elf_cals.clone();
    sorted.sort();

    return sorted;
}

pub fn part_1() -> u64 {
    let sorted = elves_by_cals_sorted();
    return sorted[sorted.len() - 1];
}

pub fn part_2() -> u64 {
    let idx = elves_by_cals_sorted().len() - 3;
    return elves_by_cals_sorted().split_at(idx).1.iter().sum();
}
