use std::collections::HashSet;

use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day9");

    let mut visited_positions : HashSet<(i32,i32)> = HashSet::new();

    let mut head_position = (0,0);
    let mut tail_position = head_position;
    visited_positions.insert(tail_position);

    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let count = line[2..].parse::<i32>().unwrap();
        for _ in 0..count {
            if direction == 'U' {
                head_position.1 += 1;
            } else if direction == 'D' {
                head_position.1 -= 1;
            } else if direction == 'L' {
                head_position.0 -= 1;
            } else if direction == 'R' {
                head_position.0 += 1;
            }
            

            // move tail to be next to head
            tail_position = get_new_tail_position(head_position, tail_position);

            visited_positions.insert(tail_position);
        }
    }

    println!("Day 9 Part 1: {}", visited_positions.len());
}

pub fn part2() {
    let lines = read_input("day9");

    let mut visited_positions : HashSet<(i32,i32)> = HashSet::new();

    let mut head_position = (0,0);
    let mut tail_positions = Vec::new();
    for _ in 0..9 {
        tail_positions.push(head_position);
    }
    visited_positions.insert(tail_positions[tail_positions.len() - 1]);

    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let count = line[2..].parse::<i32>().unwrap();

        for _ in 0..count {
            if direction == 'U' {
                head_position.1 += 1;
            } else if direction == 'D' {
                head_position.1 -= 1;
            } else if direction == 'L' {
                head_position.0 -= 1;
            } else if direction == 'R' {
                head_position.0 += 1;
            }
            
            for i in 0..tail_positions.len() {
                let mut h = head_position;
                if i > 0 {
                    h = tail_positions[i - 1];
                }
                tail_positions[i] = get_new_tail_position(h, tail_positions[i]);
            }

            visited_positions.insert(tail_positions[tail_positions.len() - 1]);
        }
    }

    println!("Day 9 Part 2: {}", visited_positions.len());
}

fn get_new_tail_position(head: (i32, i32), tail: (i32, i32)) -> (i32, i32){
    // do nothing if head and tail are adjacent or touching
    if (head.1 - tail.1).abs() <= 1 && (head.0 - tail.0).abs() <= 1 {
        return tail;
    }
    
    // if in the same row, tail moves to be next to head
    if head.0 == tail.0 {
        if head.1 > tail.1 {
            return (tail.0, head.1 - 1);
        } else {
            return (tail.0, head.1 + 1);
        }
    }
    // if in the same column, tail moves to be next to head
    if head.1 == tail.1 {
        if head.0 > tail.0 {
            return (head.0 - 1, tail.1);
        } else {
            return (head.0 + 1, tail.1);
        }
    }
    // if in different row and column, tail moves one step diagonally towards head
    if head.0 > tail.0 {
        if head.1 > tail.1 {
            return (tail.0 + 1, tail.1 + 1);
        } else {
            return (tail.0 + 1, tail.1 - 1);
        }
    } else {
        if head.1 > tail.1 {
            return (tail.0 - 1, tail.1 + 1);
        } else {
            return (tail.0 - 1, tail.1 - 1);
        }
    }
}