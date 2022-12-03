// https://adventofcode.com/2022/day/1

use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day1");

    let mut highest_sum = 0;
    let mut sum = 0;

    for line in lines {
        if line == "" {
            if sum > highest_sum {
                highest_sum = sum;
            }
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    println!("Day 1 Part 1: {}", highest_sum);
}

// https://adventofcode.com/2022/day/1#part2
pub fn part2() {
    let lines = read_input("day1");

    let mut sums : Vec<u32> = Vec::new();

    let mut sum = 0;
    for line in lines {
        if line == "" {
            sums.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<u32>().unwrap();
    }

    sums.sort_by(|a, b| b.cmp(a));

    let top_3_sum = sums[0] + sums[1] + sums[2];

    println!("Day 1 Part 2: {}", top_3_sum);
}

