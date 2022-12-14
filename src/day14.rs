use std::collections::HashMap;

use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day14");

    let mut map : HashMap<(u16, u16), char> = HashMap::new();

    for line in lines {
        let parts = line.split(" -> ").collect::<Vec<&str>>();
        let mut coords_list : Vec<(u16, u16)> = Vec::new();
        for part in parts {
            let coords = part.split(",").collect::<Vec<&str>>();
            let x = coords[0].parse::<u16>().unwrap();
            let y = coords[1].parse::<u16>().unwrap();
            coords_list.push((x, y));
        }
        draw_lines(&mut map, coords_list);
    }

    let sand_source = (500, 0);
    let mut total_units_of_sand = 0;
    while drop_sand(&mut map, sand_source, -1) {
        total_units_of_sand += 1;
    }


    println!("Day 14 Part 1: {}", total_units_of_sand);
}

fn draw_lines(map: &mut HashMap<(u16, u16), char>, coords_list : Vec<(u16, u16)>) {
    for i in 0..(coords_list.len() - 1) {
        let (start_x, start_y) = coords_list[i];
        let (end_x, end_y) = coords_list[i + 1];
        let min_x = start_x.min(end_x);
        let max_x = start_x.max(end_x);
        let min_y = start_y.min(end_y);
        let max_y = start_y.max(end_y);
        if min_x == max_x {
            for y in min_y..(max_y + 1) {
                map.insert((min_x, y), '#');
            }
        } else {
            for x in min_x..(max_x + 1) {
                map.insert((x, min_y), '#');
            }
        }
    }
}

fn drop_sand(map: &mut HashMap<(u16, u16), char>, sand_source : (u16, u16), floor: i32) -> bool {
    let (x, y) = sand_source;
    let mut x = x;
    let mut y = y;

    loop {
        if y == floor as u16 - 1  {
            // reached the floor
            map.insert((x, y), 'o');
            return true;
        }
        if y > 10000 {
            // fell into abyss
            return false;
        }
        // move down
        if !map.contains_key(&(x, y + 1)) {
            y += 1;
        }
        // otherwise move down-left
        else if !map.contains_key(&(x - 1, y + 1)) {
            x -= 1;
            y += 1;
        }
        // otherwise move down-right
        else if !map.contains_key(&(x + 1, y + 1)) {
            x += 1;
            y += 1;
        }
        // otherwise come to rest
        else {
            map.insert((x, y), 'o');
            return true;
        }
    }
}

pub fn part2() {
    let lines = read_input("day14");

    let mut map : HashMap<(u16, u16), char> = HashMap::new();

    let mut highest_y = 0;

    for line in lines {
        let parts = line.split(" -> ").collect::<Vec<&str>>();
        let mut coords_list : Vec<(u16, u16)> = Vec::new();
        for part in parts {
            let coords = part.split(",").collect::<Vec<&str>>();
            let x = coords[0].parse::<u16>().unwrap();
            let y = coords[1].parse::<u16>().unwrap();
            if y > highest_y {
                highest_y = y;
            }
            coords_list.push((x, y));
        }
        draw_lines(&mut map, coords_list);
    }

    let floor = highest_y + 2;

    let sand_source = (500, 0);
    let mut total_units_of_sand = 0;
    while drop_sand(&mut map, sand_source, floor as i32) {
        total_units_of_sand += 1;
        if map.contains_key(&sand_source) {
            break;
        }
    }

    println!("Day 14 Part 2: {}", total_units_of_sand);
}