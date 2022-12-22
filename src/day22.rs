use std::collections::HashMap;

use crate::input::read_input;

#[derive(Debug)]
enum Instruction {
    MoveForward(u16),
    TurnLeft,
    TurnRight
}

#[derive(Debug, Clone)]
struct Position {
    row: u16,
    col: u16,
    direction: (i16, i16)
}

impl Position {
    fn rotate_right(&mut self) {
        let new_direction = match self.direction {
            (0,1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => panic!("Invalid direction")
        };
        self.direction = new_direction;
    }
    fn rotate_left(&mut self) {
        let new_direction = match self.direction {
            (0,1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            _ => panic!("Invalid direction")
        };
        self.direction = new_direction;
    }
    fn do_move(&mut self, instruction : &Instruction, map: HashMap<(u16, u16), char>) -> Position {
        let map_width = *map.keys().map(|(row, _)| row).max().unwrap() + 1;
        let map_height = *map.keys().map(|(_, col)| col).max().unwrap() + 1;
        match instruction {
            Instruction::MoveForward(steps) => {
                let mut new_position = self.clone();
                let direction = self.direction;
                for _ in 0..*steps {
                    let mut next_row = (new_position.row as i16 + direction.0).rem_euclid(map_width as i16) as u16;
                    let mut next_col = (new_position.col as i16 + direction.1).rem_euclid(map_height as i16) as u16;
                    let mut char = *map.get(&(next_row, next_col)).unwrap();
                    while char == ' ' {
                        next_row = (next_row as i16 + direction.0).rem_euclid(map_width as i16) as u16;
                        next_col = (next_col as i16 + direction.1).rem_euclid(map_height as i16) as u16;
                        char = *map.get(&(next_row, next_col)).unwrap();
                    } 
                    if char == '#' {
                        break;
                    }
                    if char == '.' {
                        new_position.row = next_row;
                        new_position.col = next_col;
                    }
                }
                new_position
            },
            Instruction::TurnLeft => {
                let mut new_position = self.clone();
                new_position.rotate_left();
                new_position
            },
            Instruction::TurnRight => {
                let mut new_position = self.clone();
                new_position.rotate_right();
                new_position
            }            
        }
    }
}

pub fn part1() {
    let lines = read_input("day22");

    let mut map : HashMap<(u16, u16), char> = HashMap::new();
    let mut instructions : Vec<Instruction> = Vec::new();

    let mut i = 0;
    let length = lines.len();
    for line in lines {
        if i == length - 2 {
            i += 1;
            continue;
        }
        if i == length - 1 {
            // instructions in format 10R5L5R10L4R5L5
            let mut current_steps = "".to_string();
            for char in line.chars() {
                if char == 'L' || char == 'R' {
                    if current_steps != "" {
                        instructions.push(Instruction::MoveForward(current_steps.parse().unwrap()));
                        current_steps = "".to_string();
                    }
                    if char == 'L' {
                        instructions.push(Instruction::TurnLeft);
                    } else {
                        instructions.push(Instruction::TurnRight);
                    }
                } else {
                    current_steps = format!("{}{}", current_steps, char);
                }
            }
            if current_steps != "" {
                instructions.push(Instruction::MoveForward(current_steps.parse().unwrap()));
            }
            break;
        }
        let mut j = 0;
        for char in line.chars() {
            map.insert((i as u16, j), char);
            j += 1;
        }
        i += 1;
    }

    let map_width = *map.keys().map(|(row, _)| row).max().unwrap() + 1;
    let map_height = *map.keys().map(|(_, col)| col).max().unwrap() + 1;
    for i in 0..map_width {
        for j in 0..map_height {
            if !map.contains_key(&(i, j)) {
                map.insert((i, j), ' ');
            }
        }
    }

    let mut position = Position {
        row : 0,
        col : 0,
        direction : (0, 1)
    };
    while *map.get(&(position.row, position.col)).unwrap() == ' ' {
        position.col += 1;
    }


    for instruction in instructions {
        position = position.do_move(&instruction, map.clone());
    }

    let final_row = position.row  as usize  + 1;
    let final_col = position.col  as usize  + 1;
    let final_facing = match position.direction {
        (0,1) =>0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => panic!("Invalid direction")
    };

    let final_password = 1000 * final_row + 4*final_col + final_facing;

    println!("Final row: {}, final col: {}, final facing: {}", final_row, final_col, final_facing);

    println!("Day 22 Part 1: {}", final_password);
}