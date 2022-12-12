use std::collections::{HashMap, VecDeque};

use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day12");
    let mut elevations : HashMap<(isize, isize), u8> = HashMap::new();
    let mut start_location : (isize, isize) = (0, 0);
    let mut end_location: (isize, isize) = (0, 0);

    let mut x = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        max_x = x;
        let mut y = 0;
        for c in line.chars() {
            max_y = y;
            let location = (x, y);
            match c {
                'S' => {
                    elevations.insert(location, 0);
                    start_location = location;
                },
                'E' => {
                    elevations.insert(location, 25);
                    end_location =location;
                },
                _ => {
                    // Convert to number, a is 0, b is 1, etc.
                    elevations.insert(location, c as u8 - 97);
                }
            }
            y += 1;
        }
        x += 1;
    }

    let mut distance_map : HashMap<(isize, isize), usize> = HashMap::new();
    let mut traversal_queue : VecDeque<((isize, isize), usize)> = VecDeque::new();
    traversal_queue.push_back((start_location, 0));
    distance_map.insert(start_location, 0);


    while traversal_queue.len() > 0 {
        let next_location = traversal_queue.pop_front().unwrap();
        let ((x1, y1), distance) = next_location;
        let elevation1 = elevations.get(&(x1, y1)).unwrap();
        if (x1, y1) == end_location {
            println!("Day 12 Part 1: {}", distance);
            break;
        }
        let next_locations = vec![
            (x1 + 1, y1),
            (x1 - 1, y1),
            (x1, y1 + 1),
            (x1, y1 - 1),
        ];
        for location in next_locations {
            let (x2, y2) = location;
            if x2 < 0 || y2 < 0 || x2 > max_x || y2 > max_y {
                continue;
            }
            if distance_map.contains_key(&location) {
                continue;
            }
            let elevation2 = elevations.get(&location).unwrap();
            if elevation2 > elevation1 && elevation2 - elevation1 > 1 {
                continue;
            }
            distance_map.insert(location, distance + 1);
            traversal_queue.push_back((location, distance + 1));
        }
    }
}

pub fn part2() {
    let lines = read_input("day12");
    let mut elevations : HashMap<(isize, isize), u8> = HashMap::new();
    let mut end_location: (isize, isize) = (0, 0);

    let mut x = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        max_x = x;
        let mut y = 0;
        for c in line.chars() {
            max_y = y;
            let location = (x, y);
            match c {
                'S' => {
                    elevations.insert(location, 0);
                },
                'E' => {
                    elevations.insert(location, 25);
                    end_location =location;
                },
                _ => {
                    // Convert to number, a is 0, b is 1, etc.
                    elevations.insert(location, c as u8 - 97);
                }
            }
            y += 1;
        }
        x += 1;
    }

    let mut distance_map : HashMap<(isize, isize), usize> = HashMap::new();
    let mut traversal_queue : VecDeque<((isize, isize), usize)> = VecDeque::new();
    traversal_queue.push_back((end_location, 0));
    distance_map.insert(end_location, 0);


    while traversal_queue.len() > 0 {
        let next_location = traversal_queue.pop_front().unwrap();
        let ((x1, y1), distance) = next_location;
        let elevation1 = elevations.get(&(x1, y1)).unwrap();
        if *elevation1 == 0 {
            println!("Day 12 Part 2: {}", distance);
            break;
        }
        let next_locations = vec![
            (x1 + 1, y1),
            (x1 - 1, y1),
            (x1, y1 + 1),
            (x1, y1 - 1),
        ];
        for location in next_locations {
            let (x2, y2) = location;
            if x2 < 0 || y2 < 0 || x2 > max_x || y2 > max_y {
                continue;
            }
            if distance_map.contains_key(&location) {
                continue;
            }
            let elevation2 = elevations.get(&location).unwrap();
            if elevation2 < elevation1 && elevation1 - elevation2 > 1 {
                continue;
            }
            distance_map.insert(location, distance + 1);
            traversal_queue.push_back((location, distance + 1));
        }
    }
}
