use std::collections::{HashMap, VecDeque};

use crate::input::read_input;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32
}

impl Cube {
    fn has_common_face(&self, other: &Cube) -> bool {
        // check if the cubes have a common face
        // if they do, return true
        // if they don't, return false
        if &self.x == &other.x && &self.y == &other.y {
            return *&self.z.abs_diff(*&other.z) == 1;
        }
        if &self.x == &other.x && &self.z == &other.z {
            return *&self.y.abs_diff(*&other.y) == 1;
        }
        if &self.y == &other.y && &self.z == &other.z {
            return *&self.x.abs_diff(*&other.x) == 1;
        }
        false
    }

    fn find_adjacent_cubes(&self, others: &Vec<Cube>) -> Vec<Cube> {
        let mut adjacent_cubes : Vec<Cube> = Vec::new();
        for other in others {
            if self.has_common_face(other) {
                adjacent_cubes.push(other.clone());
            }
        }
        adjacent_cubes
    }
}

pub fn part1() {
    let lines = read_input("day18");

    let mut cubes : Vec<Cube> = Vec::new();
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;
    
    let mut map : HashMap<Cube, char> = HashMap::new();

    for line in lines {
        // line in format x,y,z
        let coords : Vec<&str> = line.split(",").collect();
        let x = coords[0].parse::<i32>().unwrap();
        let y = coords[1].parse::<i32>().unwrap();
        let z = coords[2].parse::<i32>().unwrap();
        if x < min_x { min_x = x; }
        if y < min_y { min_y = y; }
        if z < min_z { min_z = z; }
        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }
        if z > max_z { max_z = z; }
        cubes.push(Cube { x, y, z });
        map.insert(Cube { x, y, z }, '#');
    }

    let mut total_sides = cubes.len() * 6;

    for i in 0..cubes.len() {
        for j in i+1..cubes.len() {
            if cubes[i].has_common_face(&cubes[j]) {
                total_sides -= 2;
            }
        }
    }

    println!("Day 18 Part 1: {}", total_sides);

    let mut exterior_spaces : VecDeque<Cube> = VecDeque::new();
    exterior_spaces.push_back(Cube { x: min_x - 1, y: min_y - 1, z: min_z - 1 });

    // search for exterior spaces
    while !exterior_spaces.is_empty() {
        let current = exterior_spaces.pop_front().unwrap();
        if map.contains_key(&current) {        
            continue;
        }
        map.insert(current.clone(), '.');
        if current.x < max_x + 1 {
            exterior_spaces.push_back(Cube { x: current.x + 1, y: current.y, z: current.z });
        }
        if current.y < max_y + 1 {
            exterior_spaces.push_back(Cube { x: current.x, y: current.y + 1, z: current.z });
        }
        if current.z < max_z + 1 {
            exterior_spaces.push_back(Cube { x: current.x, y: current.y, z: current.z + 1 });
        }
        if current.x > min_x - 1 {
            exterior_spaces.push_back(Cube { x: current.x - 1, y: current.y, z: current.z });
        };
        if current.y > min_y - 1 {
            exterior_spaces.push_back(Cube { x: current.x, y: current.y - 1, z: current.z });
        }
        if current.z > min_z - 1 {
            exterior_spaces.push_back(Cube { x: current.x, y: current.y, z: current.z - 1 });
        }
    }

    // search for interior spaces
    let mut interior_spaces : Vec<Cube> = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if !map.contains_key(&Cube { x, y, z }) {
                    interior_spaces.push(Cube { x, y, z });
                }
            }
        }
    }

    // for each interior space, find adjacent cubes
    for interior_space in interior_spaces {
        let adjacent_cubes = interior_space.find_adjacent_cubes(&cubes);
        total_sides -= adjacent_cubes.len();
    }



    println!("Day 18 Part 2: {}", total_sides);
}