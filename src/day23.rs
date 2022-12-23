use std::collections::{HashMap, HashSet};

use crate::input::read_input;

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn to_position(&self, position: (isize, isize)) -> (isize, isize) {
        let (i, j) = position;
        match self {
            Direction::North => (i - 1, j),
            Direction::South => (i + 1, j),
            Direction::West => (i, j - 1),
            Direction::East => (i, j + 1),
        }
    }
    fn adjacent_positions(&self, position: (isize, isize)) -> Vec<(isize, isize)> {
        let n = Direction::North.to_position(position);
        let s = Direction::South.to_position(position);
        let w = Direction::West.to_position(position);
        let e = Direction::East.to_position(position);
        let nw = Direction::North.to_position(w);
        let ne = Direction::North.to_position(e);
        let sw = Direction::South.to_position(w);
        let se = Direction::South.to_position(e);
        match self {
            Direction::North => vec![n, nw, ne],
            Direction::South => vec![s, sw, se],
            Direction::West => vec![w, nw, sw],
            Direction::East => vec![e, ne, se],
        }
    }
}


pub fn part1() {
    let lines = read_input("day23");

    let mut map : HashSet<(isize, isize)> = HashSet::new();
    let mut i = 0;
    for line in lines {
        let mut j = 0;
        for c in line.chars() {
            if c == '#' {
                map.insert((i, j));
            }
            j += 1;
        }
        i += 1;
    }

    let mut directions : Vec<Direction> = Vec::new();
    let mut elf_moved = false;
    directions.push(Direction::North);
    directions.push(Direction::South);
    directions.push(Direction::West);
    directions.push(Direction::East);

    for round in 0..10 {
        (map, directions, elf_moved) = do_round(&map, &directions);
    }

    let empty_ground_tiles=count_empty_ground_tiles(&map);

    println!("Day 23 Part 1: {}", empty_ground_tiles);

    let mut rounds = 10;

    while elf_moved {
        (map, directions, elf_moved) = do_round(&map, &directions);
        rounds += 1;
    }
    println!("Day 23 Part 2: {}", rounds);
}

fn do_round(map: &HashSet<(isize, isize)>, directions: &Vec<Direction>) -> (HashSet<(isize, isize)>, Vec<Direction>, bool) {
    let mut map = map.clone();
    let mut directions = directions.clone();
    let mut proposed_moves : HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut elf_moved = false;
    // first half of the round
    for elf in &map {
        // continue if all spaces around the elf are empty
        let mut all_empty = true;
        for direction in &directions {
            let adjacent_positions = direction.adjacent_positions(*elf);
            // continue if any of the adjacent positions are filled
            if adjacent_positions.iter().any(|p| map.contains(p)) {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            continue;
        }
        for direction in &directions {
            let adjacent_positions = direction.adjacent_positions(*elf);
            // continue if any of the adjacent positions are filled
            if adjacent_positions.iter().any(|p| map.contains(p)) {
                continue;
            }
            proposed_moves.insert(*elf, direction.to_position(*elf));
            break;
        }
    }
    // second half of the round
    for (elf, new_position) in &proposed_moves {
        // continue if another elf is moving to the same position
        let mut other_elf_moving = false;
        for (other_elf, other_new_position) in &proposed_moves {
            if other_elf != elf && other_new_position == new_position {
                other_elf_moving = true;
                break;
            }
        }
        if other_elf_moving {
            continue;
        }
        elf_moved = true;
        map.remove(elf);
        map.insert(*new_position);
    }
    // cycle the directions
    let first_direction = directions.remove(0);
    directions.push(first_direction);
    return (map, directions, elf_moved);
}

fn print_map(map : &HashSet<(isize, isize)>) {
    println!();
    let mut min_i = 0;
    let mut max_i = 0;
    let mut min_j = 0;
    let mut max_j = 0;
    for (i, j) in map {
        if *i < min_i {
            min_i = *i;
        }
        if *i > max_i {
            max_i = *i;
        }
        if *j < min_j {
            min_j = *j;
        }
        if *j > max_j {
            max_j = *j;
        }
    }
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if map.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn count_empty_ground_tiles(map : &HashSet<(isize, isize)>) -> i32 {
    let mut min_i = 0;
    let mut max_i = 0;
    let mut min_j = 0;
    let mut max_j = 0;
    for (i, j) in map {
        if *i < min_i {
            min_i = *i;
        }
        if *i > max_i {
            max_i = *i;
        }
        if *j < min_j {
            min_j = *j;
        }
        if *j > max_j {
            max_j = *j;
        }
    }
    let mut count = 0;
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if !map.contains(&(i, j)) {
               count += 1;
            }
        }
    }
    count
}