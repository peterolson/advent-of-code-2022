use std::collections::{HashMap, HashSet, VecDeque};

use crate::input::read_input;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn part1() {
    let lines = read_input("day24");

    let mut walls : HashSet<(i32, i32)> = HashSet::new();
    let mut blizzards : Vec<(i32, i32, Direction)> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;

    let mut i = 0;
    for line in lines {
        let mut j = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    walls.insert((i, j));
                },
                '<' => {
                    blizzards.push((i, j, Direction::Left));
                },
                '>' => {
                    blizzards.push((i, j, Direction::Right));
                },
                '^' => {
                    blizzards.push((i, j, Direction::Up));
                },
                'v' => {
                    blizzards.push((i, j, Direction::Down));
                },
                _ => {}
            }
            j += 1;
            cols = j;
        }
        i += 1;
        rows = i;
    }

    let mut blizzard_positions : Vec<Vec<(i32, i32, Direction)>> = Vec::new();
    blizzard_positions.push(blizzards.clone());

    let (i, j, minutes) = search(&mut blizzard_positions, rows, cols, &walls, (0, 1, 0), rows - 1);

    println!("Day 24 Part 1: {}", minutes);

    let (i, j, minutes) = search(&mut blizzard_positions, rows, cols, &walls, (i, j, minutes), 0);

    println!("Back to start: {}", minutes);

    let (i, j, minutes) = search(&mut blizzard_positions, rows, cols, &walls, (i, j, minutes), rows - 1);

    println!("Day 24 Part 2: {}", minutes);
}

fn search(blizzard_positions: &mut Vec<Vec<(i32, i32, Direction)>>, rows: i32, cols: i32, walls: &HashSet<(i32, i32)>, initial_position : (i32, i32, i32), destination_row : i32) -> (i32, i32, i32) {
    let mut search_queue : VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut encountered_states : HashSet<(i32, i32, i32)> = HashSet::new();
    search_queue.push_back(initial_position);

    let mut best_time = i32::MAX;
    let mut best_position = initial_position;

    while search_queue.len() > 0 {
        let (i, j, minutes) = search_queue.pop_front().unwrap();
        if encountered_states.contains(&(i, j, minutes)) {
            continue;
        }
        encountered_states.insert((i, j, minutes));

        if minutes >= best_time {
            continue;
        }
        if i == destination_row {
            best_time = minutes;
            best_position = (i, j, minutes);
            continue;
        }

        if blizzard_positions.len() <= minutes as usize + 1 {
            blizzard_positions.push(get_next_blizzard_positions(&blizzard_positions[minutes as usize], rows, cols));
        }
        let blizzards = &blizzard_positions[minutes as usize + 1];
        let possible_positions : Vec<(i32, i32)> = vec![(i-1, j), (i+1, j), (i, j-1), (i, j+1), (i, j)];
        for position in possible_positions {
            let (i, j) = position;
            if blizzards.iter().any(|(x, y, _)| *x == i && *y == j) {
                continue;
            }
            if walls.contains(&position) {
                continue;
            }
            if i < 0 || j < 0 {
                continue;
            }
            if i >= rows || j >= cols {
                continue;
            }
            search_queue.push_back((i, j, minutes + 1));
        }

    }
    best_position
}

fn get_next_blizzard_positions(blizzards : &Vec<(i32, i32, Direction)>, rows: i32, cols: i32) -> Vec<(i32, i32, Direction)> {
    let mut new_positions : Vec<(i32, i32, Direction)> = Vec::new();
    for blizzard in blizzards {
        let (i,j, direction) = blizzard;
        let i = *i;
        let j = *j;
        match direction {
            Direction::Left => {
                if j == 1 {
                    new_positions.push((i, cols - 2, Direction::Left));
                } else {
                    new_positions.push((i, j-1, Direction::Left));
                }
            },
            Direction::Right => {
                if j == cols - 2 {
                    new_positions.push((i, 1, Direction::Right));
                } else {
                    new_positions.push((i, j+1, Direction::Right));
                }
            },
            Direction::Up => {
                if i == 1 {
                    new_positions.push((rows - 2, j, Direction::Up));
                } else {
                    new_positions.push((i-1, j, Direction::Up));
                }
            },
            Direction::Down => {
                if i == rows - 2 {
                    new_positions.push((1, j, Direction::Down));
                } else {
                    new_positions.push((i+1, j, Direction::Down));
                }
            },
        }
    }
    new_positions
}

fn print_state(blizzards : &Vec<(i32, i32, Direction)>, walls: &HashSet<(i32, i32)>, position: (i32, i32), rows: i32, cols: i32) {
    println!("position: {:?}", position);
    for i in 0..rows {
        for j in 0..cols {
            if blizzards.iter().any(|(x, y, _)| *x == i && *y == j) {
                let count = blizzards.iter().filter(|(x, y, _)| *x == i && *y == j).count();
                if count > 1 {
                    print!("{}", count);
                    continue;
                }
                let first = blizzards.iter().find(|(x, y, _)| *x == i && *y == j).unwrap();
                let direction = &first.2;
                match direction {
                    Direction::Left => {
                        print!("<");
                    },
                    Direction::Right => {
                        print!(">");
                    },
                    Direction::Up => {
                        print!("^");
                    },
                    Direction::Down => {
                        print!("v");
                    },
                }
            } else if walls.contains(&(i, j)) {
                print!("#");
            } else if position.0 == i && position.1 == j {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}