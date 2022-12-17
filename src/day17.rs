use std::collections::{HashMap};

use crate::input::read_input;

#[derive(Debug)]
struct Rock {
    width: u8,
    height: u8,
    chars: Vec<char>
}

pub fn part1() {
    let lines = read_input("day17");

    let rock1 = Rock {
        width: 4,
        height: 1,
        chars: "####".chars().collect()
    };
    let rock2 = Rock {
        width: 3,
        height: 3,
        chars: ".#.###.#.".chars().collect()
    };
    let rock3 = Rock {
        width: 3,
        height: 3,
        chars: "..#..####".chars().collect()
    };
    let rock4 = Rock {
        width: 1,
        height: 4,
        chars: "####".chars().collect()
    };
    let rock5 = Rock {
        width: 2,
        height: 2,
        chars: "####".chars().collect()
    };

    let rocks = vec![rock1, rock2, rock3, rock4, rock5];

    let mut jet_pattern : Vec<i64> = vec![];
    for char in lines[0].chars() {
        if char == '>' {
            jet_pattern.push(1);
        } else if char == '<' {
            jet_pattern.push(-1);
        }
    }

    println!("Day 17 Part 1: {}", get_height(&rocks, &jet_pattern, 2022));

    let limit : i64 = 1000000000000;

    let (prelude_length, prelude_height, cycle_length, repeating_heights) = get_cycle_length(&rocks, &jet_pattern);

    let limit_after_prelude = limit - prelude_length as i64;
    let cycles = limit_after_prelude / cycle_length as i64;
    let remainder = limit_after_prelude % cycle_length as i64;

    let total_height = prelude_height + cycles * repeating_heights[repeating_heights.len() - 1] + repeating_heights[remainder as usize];

    println!("Day 17 Part 2: {}", total_height);
}

fn get_height(rocks: &Vec<Rock>, jet_pattern: &Vec<i64>, max_rock_count : usize) -> i64 {
    let mut map : HashMap<(i64, i64), char> = HashMap::new();
    let mut highest_y : i64 = 0;
    let mut rock_count = 0;
    let mut iteration = 0;

    while rock_count < max_rock_count {
        let rock = &rocks[rock_count % rocks.len()];
        rock_count += 1;

        let mut y = highest_y + rock.height as i64 + 3;
        let mut x : i64 = 2;

        loop {
            let direction = jet_pattern[iteration % jet_pattern.len()];
            iteration += 1;
            if !collides(rock, x + direction, y, &map) {
                x += direction;
            }
            if !collides(rock, x, y - 1, &map) {
                y -= 1;
            } else {
                add_rock_to_map(rock, x, y, &mut map);
                if y > highest_y {
                    highest_y = y;
                }
                break;
            }
        }
    }
    highest_y
}


fn get_cycle_length(rocks: &Vec<Rock>, jet_pattern: &Vec<i64>) -> (usize, i64, usize, Vec<i64>) {
    let mut map : HashMap<(i64, i64), char> = HashMap::new();
    let mut highest_y : i64 = 0;
    let mut rock_count = 0;
    let mut iteration = 0;

    let mut encountered_states : Vec<(i64, i64)> = Vec::new();
    let mut encountered_state_counts : HashMap<(i64, i64), usize> = HashMap::new();
    let mut highest_y_list : Vec<i64> = vec![];
    let mut previous_highest_y = highest_y;

    loop {
        let rock_index = rock_count % rocks.len();
        let iteration_index = iteration % jet_pattern.len();
        let state = (rock_index as i64, iteration_index as i64);
        let mut count = 0;
        if encountered_state_counts.contains_key(&state) {
            count = *encountered_state_counts.get(&state).unwrap();
        }
        if count > 1 {
            encountered_states.push(state);
            break;
        }
        encountered_states.push(state);
        encountered_state_counts.insert(state, count + 1);
        highest_y_list.push(highest_y - previous_highest_y);
        previous_highest_y = highest_y;
        let rock = &rocks[rock_index];
        rock_count += 1;

        let mut y = highest_y + rock.height as i64 + 3;
        let mut x : i64 = 2;

        loop {
            let direction = jet_pattern[iteration % jet_pattern.len()];
            iteration += 1;
            if !collides(rock, x + direction, y, &map) {
                x += direction;
            }
            if !collides(rock, x, y - 1, &map) {
                y -= 1;
            } else {
                add_rock_to_map(rock, x, y, &mut map);
                if y > highest_y {
                    highest_y = y;
                }
                break;
            }
        }
    }

    let repeated_state = encountered_states[encountered_states.len() - 1];
    let repeated_state_index = encountered_states.iter().position(|&x| x == repeated_state).unwrap();
    let repeated_state_index = repeated_state_index + 1 + encountered_states[repeated_state_index + 1..].iter().position(|&x| x == repeated_state).unwrap();
    let cycle_length = encountered_states.len() - repeated_state_index - 1;
    let prelude_length = repeated_state_index;
    // sum up to prelude length
    let prelude_height = highest_y_list[0..repeated_state_index].iter().sum();
    let mut repeating_heights : Vec<i64> = highest_y_list[repeated_state_index..].to_vec();
    // convert to cumulative sum
    let mut cumulative_sum = 0;
    for i in 0..repeating_heights.len() {
        cumulative_sum += repeating_heights[i];
        repeating_heights[i] = cumulative_sum;
    }

    (prelude_length, prelude_height, cycle_length, repeating_heights)
}

fn collides(rock: &Rock, x: i64, y: i64, map : &HashMap<(i64, i64), char>) -> bool {
    for i in 0..rock.width {
        for j in 0..rock.height {
            let char = rock.chars[(i + j * rock.width) as usize];
            if char == '.' {
                continue;
            }
            let x = x + i as i64;
            let y = y - j as i64;
            if x < 0 {
                return true;
            }
            if x > 6 {
                return true;
            }
            if y <= 0 {
                return true;
            }
            if map.contains_key(&(x, y)) {
                if *map.get(&(x, y)).unwrap() == '#' {
                    return true;
                }
            }
        }
    }
    false
}

fn add_rock_to_map(rock: &Rock, x: i64, y: i64, map : &mut HashMap<(i64, i64), char>) {
    for i in 0..rock.width {
        for j in 0..rock.height {
            let char = rock.chars[(i + j * rock.width) as usize];
            if char == '.' {
                continue;
            }
            let x = x + i as i64;
            let y = y - j as i64;
            map.insert((x, y), char);
        }
    }
}