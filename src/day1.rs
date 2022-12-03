use std::fs;

fn main() {
    let data = fs::read_to_string("data/day1.txt").unwrap();

    let mut elves: Vec<i32> = Vec::new();
    elves.push(0);

    for line in data.split("\n") {
        if line.trim().is_empty() {
            elves.push(0);
        }
        else {
            let value = line.trim().parse::<i32>();
            *elves.last_mut().unwrap() += value.unwrap();
        }
    }

    elves.sort();
    println!("Part 1: {}", elves.last().unwrap());
    println!("Part 2: {}", elves.iter().rev().take(3).sum::<i32>());
}