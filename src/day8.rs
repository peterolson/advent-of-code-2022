use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day8");

    // 2-dimensional array of numbers
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut row: Vec<i32> = Vec::new();
        for number in line.trim().chars() {
            row.push(number.to_digit(10).unwrap() as i32);
        }
        numbers.push(row);
    }

    // find the visible numbers
    let mut visible = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers[0].len() {
            if is_visible(numbers.clone(), i, j) {
                visible += 1;
            }
        }
    }

    println!("Day 8 Part 1: {}", visible);
}

fn is_visible(numbers: Vec<Vec<i32>>, i : usize, j : usize) -> bool {
    // always visible if on edge
    if i == 0 || i == numbers.len() - 1 || j == 0 || j == numbers[0].len() - 1 {
        return true;
    }
    // check if taller than all neightbors on right
    let mut taller = true;
    for k in 0..i {
        if numbers[k][j] >= numbers[i][j] {
            taller = false;
            break;
        }
    }
    if taller {
        return true;
    }
    // check if taller than all neightbors on left
    taller = true;
    for k in i+1..numbers.len() {
        if numbers[k][j] >= numbers[i][j] {
            taller = false;
            break;
        }
    }
    if taller {
        return true;
    }
    // check if taller than all neightbors above
    taller = true;
    for k in 0..j {
        if numbers[i][k] >= numbers[i][j] {
            taller = false;
            break;
        }
    }
    if taller {
        return true;
    }
    // check if taller than all neightbors below
    taller = true;
    for k in j+1..numbers[0].len() {
        if numbers[i][k] >= numbers[i][j] {
            taller = false;
            break;
        }
    }
    if taller {
        return true;
    }
    return false;
}

pub fn part2() {
    let lines = read_input("day8");

    // 2-dimensional array of numbers
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut row: Vec<i32> = Vec::new();
        for number in line.trim().chars() {
            row.push(number.to_digit(10).unwrap() as i32);
        }
        numbers.push(row);
    }

    // find the visible numbers
    let mut max_scenic_score = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers[0].len() {
            let scenic_score = get_scenic_score(numbers.clone(), i, j);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("Day 8 Part 2: {}", max_scenic_score);
}

fn get_scenic_score(numbers: Vec<Vec<i32>>, i : usize, j : usize) -> u32 {
    let mut left_shorter_count = 0;

    for k in 1..(i+1) {
        if numbers[i - k][j] < numbers[i][j] {
            left_shorter_count += 1;
        } else {
            left_shorter_count += 1;
            break;
        }
    }

    let mut right_shorter_count = 0;

    for k in i+1..numbers.len() {
        if numbers[k][j] < numbers[i][j] {
            right_shorter_count += 1;
        } else {
            right_shorter_count += 1;
            break;
        }
    }

    let mut above_shorter_count = 0;

    for k in 1..(j+1) {
        if numbers[i][j - k] < numbers[i][j] {
            above_shorter_count += 1;
        } else {
            above_shorter_count += 1;
            break;
        }
    }

    let mut below_shorter_count = 0;

    for k in j+1..numbers[0].len() {
        if numbers[i][k] < numbers[i][j] {
            below_shorter_count += 1;
        } else {
            below_shorter_count += 1;
            break;
        }
    }

    return left_shorter_count * right_shorter_count * above_shorter_count * below_shorter_count;
}