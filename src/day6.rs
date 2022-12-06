use crate::input::read_input;


pub fn part1() {
    let lines = read_input("day6");

    let str = &lines[0];
    for i in 3..str.len() {
        let last4Chars = &str[i-3..i+1];
        // check if all chars are different
        let mut all_different = true;
        for j in 0..last4Chars.len() {
            for k in 0..last4Chars.len() {
                if j != k && last4Chars.chars().nth(j) == last4Chars.chars().nth(k) {
                    all_different = false;
                }
            }
        }
        if all_different {
            println!("Day 6 Part 1: {}", i + 1);
            break;
        }
    }
    
}

pub fn part2() {
    let lines = read_input("day6");

    let str = &lines[0];
    for i in 13..str.len() {
        let last14_chars = &str[i-13..i+1];
        // check if all chars are different
        let mut all_different = true;
        for j in 0..last14_chars.len() {
            for k in 0..last14_chars.len() {
                if j != k && last14_chars.chars().nth(j) == last14_chars.chars().nth(k) {
                    all_different = false;
                }
            }
        }
        if all_different {
            println!("Day 6 Part 2: {}", i + 1);
            break;
        }
    }
}