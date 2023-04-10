mod util;
mod day1;
mod day2;
mod day3;
mod day5;

fn main() {
    println!("\n------------------------ Day 1 -------------------------");
    println!("Day 1, Part 1: {}", day1::part_1());
    println!("Day 1, Part 2: {}", day1::part_2());

    println!("\n------------------------ Day 2 -------------------------");
    println!("Day 2, Part 1: {}", day2::part_1());
    println!("Day 2, Part 2: {}", day2::part_2());

    println!("\n------------------------ Day 3 -------------------------");
    println!("Day 3, Part 1: {}", day3::part_1());
    println!("Day 3, Part 2: {}", day3::part_2());

    println!("\n------------------------ Day 5 -------------------------");
    println!("Day 5, Part 1: {:?}", day5::part_1());
    println!("Day 5, Part 2: {:?}", day5::part_2());
}
