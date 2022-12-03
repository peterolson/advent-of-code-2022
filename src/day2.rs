
use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day2");
    let mut total_score = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let a = parts[0];
        let b = parts[1];
        let score = get_score(a, b);
        total_score += score;
    }
    println!("Day 2 Part 1: {}", total_score);
}

pub fn part2() {
    let lines = read_input("day2");
    let mut total_score = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let a = parts[0];
        let b = get_b(a, parts[1]);
        let score = get_score(a, b);
        total_score += score;
    }
    println!("Day 2 Part 2: {}", total_score);
}

fn get_score(a : &str, b : &str) -> u32 {
    let mut score = 0;
    if b == "X" {
        score += 1;
    }
    if b == "Y" {
        score += 2;
    }
    if b == "Z" {
        score += 3;
    }

    // draw
    if a == "A" && b == "X" ||
       a == "B" && b == "Y" ||
       a == "C" && b == "Z" 
    {
        score += 3;
    }
   
    else if a == "A" && b == "Y" || // paper beats rock
            a == "B" && b == "Z" || // scissors beats paper
            a == "C" && b == "X"    // rock beats scissors
    {
        score += 6;
    }
    
    
    score
}

fn get_b<'life>(a : &str, b: &str) -> &'life str {
    if a == "A" {
        if b == "X" {
            return "Z";
        }
        if b == "Y" {
            return "X";
        }
       return "Y";
    }
    if a == "B" {
        if b == "X" {
            return "X";
        }
        if b == "Y" {
            return "Y";
        }
       return "Z";
    }
    if b == "X" {
        return "Y";
    }
    if b == "Y" {
        return "Z";
    }
    return "X";
}