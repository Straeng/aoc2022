use std::fs;
use phf::phf_map;

// Rock:    X,A
// Paper:   Y,B
// Scissor: Z,C

static POINTS: phf::Map<&str, u32> = phf_map! {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
};

static LOOSE: phf::Map<&str, &str> = phf_map! {
    "A" => "Z",
    "B" => "X",
    "C" => "Y",
};

static WIN: phf::Map<&str, &str> = phf_map! {
    "A" => "Y",
    "B" => "Z",
    "C" => "X",
};

static DRAW: phf::Map<&str, &str> = phf_map! {
    "A" => "X",
    "B" => "Y",
    "C" => "Z",
};

fn main() {
    let data = fs::read_to_string("data/day2.txt").unwrap();    

    let mut score1 : u32 = 0;
    let mut score2 : u32 = 0;

    for line in data.split("\n") {
        let  (a, b) = line.split_once(" ").unwrap();
        score1 += eval_round(a, b);
        score2 += eval_round(a, follow_instruction(a, b));
    }
    println!("Part1: {score1}");
    println!("Part2: {score2}");
}


fn eval_round(his_move: &str, my_move: &str) -> u32 {
    let mut points = POINTS[my_move];
    if WIN[his_move] == my_move {
        points += 6;
    }
    else if DRAW[his_move] == my_move {
        points += 3;
    }

    return points;
}

fn follow_instruction(his_move: &str, instruction: &str) -> &'static str {
    if instruction == "X" {
        return LOOSE[his_move];
    }
    else if instruction == "Y" {
        return DRAW[his_move];
    }
    else {
        return WIN[his_move];
    }
}
