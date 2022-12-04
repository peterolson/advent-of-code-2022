use std::collections::HashSet;

use crate::input::read_input;


pub fn part1() {
    let lines = read_input("day4");

    let mut count = 0;

    for line in lines {
        let pair = line.split(",").collect::<Vec<&str>>();
        let first = pair[0];
        let second = pair[1];
        let first_set = range_to_set(first);
        let second_set = range_to_set(second);
        let contains = set_fully_contains(&first_set, &second_set) || set_fully_contains(&second_set, &first_set);
        if contains {
            count += 1;
        }

    }

    println!("Day 4 Part 1: {}", count);
}

pub fn part2() {
    let lines = read_input("day4");

    let mut count = 0;

    for line in lines {
        let pair = line.split(",").collect::<Vec<&str>>();
        let first = pair[0];
        let second = pair[1];
        let first_set = range_to_set(first);
        let second_set = range_to_set(second);
        let contains = has_overlap(&first_set, &second_set);
        if contains {
            count += 1;
        }

    }

    println!("Day 4 Part 2: {}", count);
}

fn range_to_set(range: &str) -> HashSet<i32> {
    let bounds = range.split("-").collect::<Vec<&str>>();
    let start = bounds[0].parse::<i32>().unwrap();
    let end = bounds[1].parse::<i32>().unwrap();
    let mut set = HashSet::new();
    for i in start..(end + 1) {
        set.insert(i);
    }
    set
}

fn set_fully_contains(set1 : &HashSet<i32>, set2 : &HashSet<i32>) -> bool {
    for i in set1 {
        if !set2.contains(i) {
            return false;
        }
    }
    true
}

fn has_overlap(set1 : &HashSet<i32>, set2 : &HashSet<i32>) -> bool {
    for i in set1 {
        if set2.contains(i) {
            return true;
        }
    }
    false
}