use std::fs;

fn main() {
    let data = fs::read_to_string("data/day1.txt").unwrap();

    println!("Part 2: {}", elves.iter().rev().take(3).sum::<i32>());
}