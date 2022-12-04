use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day3");
    let mut total_priority = 0;

    for line in lines {
        let length = line.len();
        let first_half = &line[0..length/2];
        let second_half = &line[length/2..length];
        total_priority += get_priority(first_half, second_half);
        
    }
    println!("Day 3 Part 1: {}", total_priority);
}

pub fn part2() {
    let lines = read_input("day3");

    // go through lines in groups of 3
    let mut total_priority = 0;

    for i in (0..lines.len()).step_by(3) {
        let line1 = &lines[i];
        let line2 = &lines[i+1];
        let line3 = &lines[i+2];

        // get characters that appear in all three lines
        let mut chars = line1.chars().collect::<Vec<char>>();
        chars.retain(|&c| line2.contains(c) && line3.contains(c));

        let char = chars[0];
        let mut priority = 0;
        // a-z have priority 1-26, A-Z have priority 27-52
        if char.is_ascii_lowercase() {
            priority += char as u32 - 96;
        }
        else {
            priority += char as u32 - 38;
        }
        total_priority += priority;
    }
    println!("Day 3 Part 2: {}", total_priority);
}

fn get_priority(first_half:&str, second_half : &str) -> u32 {
    for i in 0..first_half.len() {
        for j in 0..second_half.len() {
            if first_half.chars().nth(i).unwrap() == second_half.chars().nth(j).unwrap() {
                let common_char = first_half.chars().nth(i).unwrap();
                let mut priority = 0;
                // a-z have priority 1-26, A-Z have priority 27-52
                if common_char.is_ascii_lowercase() {
                    priority += common_char as u32 - 96;
                }
                else {
                    priority += common_char as u32 - 38;
                }
                return priority;
            }
        }
    }
    0
}