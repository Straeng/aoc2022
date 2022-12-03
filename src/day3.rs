use std::fs;
use std::collections::HashSet;

fn main() {
    let data = fs::read_to_string("data/day3.txt").unwrap();

    part1(&data);
    part2(&data);
}

fn part1(data: &String) {
    let mut sum = 0;
    for line in data.split_whitespace() {
        let n = line.chars().count();
        let c1: HashSet<char> = line[0..n/2].chars().collect();
        let c2: HashSet<char> = line[n/2..n].chars().collect();

        let mut intersect = c1.intersection(&c2);
        let c = intersect.next().unwrap();
        sum += char_value(c);
    }

    println!("{}", sum);
}

fn part2(data: &String) {
    let mut sum = 0;
    for chunk in data.split_whitespace().collect::<Vec<_>>().chunks(3) {
        let badge = chunk.into_iter().map(|x| x.chars().collect::<HashSet<_>>()).reduce(|a, b| a.intersection(&b).cloned().collect()).unwrap();
        let c = badge.into_iter().next().unwrap();
        sum += char_value(&c);
    }
    println!("{}", sum);
}

fn char_value(c: &char) -> u32 {
    if c.is_uppercase() {
        return c.clone() as u32 - 'A' as u32 + 27;
    }
    else {
        return c.clone() as u32 - 'a' as u32 + 1;
    }
}