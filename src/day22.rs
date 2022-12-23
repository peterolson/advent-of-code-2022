use std::collections::HashMap;

use crate::input::read_input;

#[derive(Debug)]
enum Instruction {
    MoveForward(u16),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone)]
struct Position {
    row: u16,
    col: u16,
    direction: (i16, i16),
}

impl Position {
    fn rotate_right(&mut self) {
        let new_direction = match self.direction {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => panic!("Invalid direction"),
        };
        self.direction = new_direction;
    }
    fn rotate_left(&mut self) {
        let new_direction = match self.direction {
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            _ => panic!("Invalid direction"),
        };
        self.direction = new_direction;
    }
    fn do_move(&mut self, instruction: &Instruction, map: HashMap<(u16, u16), char>) -> Position {
        let map_width = *map.keys().map(|(row, _)| row).max().unwrap() + 1;
        let map_height = *map.keys().map(|(_, col)| col).max().unwrap() + 1;
        match instruction {
            Instruction::MoveForward(steps) => {
                let mut new_position = self.clone();
                let direction = self.direction;
                for _ in 0..*steps {
                    let mut next_row =
                        (new_position.row as i16 + direction.0).rem_euclid(map_width as i16) as u16;
                    let mut next_col = (new_position.col as i16 + direction.1)
                        .rem_euclid(map_height as i16)
                        as u16;
                    let mut char = *map.get(&(next_row, next_col)).unwrap();
                    while char == ' ' {
                        next_row =
                            (next_row as i16 + direction.0).rem_euclid(map_width as i16) as u16;
                        next_col =
                            (next_col as i16 + direction.1).rem_euclid(map_height as i16) as u16;
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
            }
            Instruction::TurnLeft => {
                let mut new_position = self.clone();
                new_position.rotate_left();
                new_position
            }
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

    let mut map: HashMap<(u16, u16), char> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

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
        row: 0,
        col: 0,
        direction: (0, 1),
    };
    while *map.get(&(position.row, position.col)).unwrap() == ' ' {
        position.col += 1;
    }

    for instruction in &instructions {
        position = position.do_move(&instruction, map.clone());
    }

    let final_row = position.row as usize + 1;
    let final_col = position.col as usize + 1;
    let final_facing = match position.direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => panic!("Invalid direction"),
    };

    let final_password = 1000 * final_row + 4 * final_col + final_facing;

    println!("Day 22 Part 1: {}", final_password);

    part2(&map, &instructions);
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn perpendicular(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Left, Direction::Right],
            Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Up, Direction::Down],
            Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    fn to_tuple(&self) -> (i16, i16) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct CubeFace {
    id: u8,
    connections: HashMap<Direction, (u8, Direction)>,
    map: HashMap<(u16, u16), char>,
}

#[derive(Debug, Clone)]
struct CubePosition {
    id: u8,
    position: Position
}

impl CubePosition {
    fn do_move(&self, instruction: &Instruction, face_width: u16,  cube_id: u8, cube_faces : &HashMap<u8, CubeFace>) -> CubePosition {
        match instruction {
            Instruction::MoveForward(steps) => {
                let mut new_position = self.clone();
                let mut cube_face = cube_faces.get(&cube_id).unwrap();
                let mut map = &cube_face.map;
                let mut cube_id = cube_id;
                let mut direction = new_position.position.direction;
                for _ in 0..*steps {
                    let mut next_row = new_position.position.row as i16 + new_position.position.direction.0;
                    let mut  next_col = new_position.position.col as i16 + new_position.position.direction.1;
                    if next_row < 0 {
                        // up
                        let (destination_id, destination_direction) = cube_face.connections.get(&Direction::Up).unwrap();
                        cube_id = *destination_id;
                        cube_face = cube_faces.get(&cube_id).unwrap();
                        map = &cube_face.map;
                        match *destination_direction {
                            Direction::Down => {
                                next_row = face_width as i16 - 1;
                            }
                            Direction::Up => {
                                next_row = 0;
                                next_col = face_width as i16 - next_col - 1;
                            }
                            Direction::Left => {
                                next_row = next_col;
                                next_col = 0;
                            }
                            Direction::Right => {
                                next_row = face_width as i16 - next_col - 1;
                                next_col = face_width as i16 - 1;

                            }
                        }
                        direction = destination_direction.opposite().to_tuple();
                    }
                    if next_row >= face_width as i16 {
                        // down
                        let (destination_id, destination_direction) = cube_face.connections.get(&Direction::Down).unwrap();
                        cube_id = *destination_id;
                        cube_face = cube_faces.get(&cube_id).unwrap();
                        map = &cube_face.map;
                        match *destination_direction {
                            Direction::Down => {
                                next_row = face_width as i16 - 1;
                                next_col = face_width as i16 - next_col - 1;
                            }
                            Direction::Up => {
                                next_row = 0;
                            }
                            Direction::Left => {
                                next_row = face_width as i16 - next_col - 1;
                                next_col = 0;
                            }
                            Direction::Right => {
                                next_row = next_col;
                                next_col = face_width as i16 - 1;

                            }
                        }
                        direction = destination_direction.opposite().to_tuple();
                    }
                    if next_col < 0 {
                        // left
                        let (destination_id, destination_direction) = cube_face.connections.get(&Direction::Left).unwrap();
                        cube_id = *destination_id;
                        cube_face = cube_faces.get(&cube_id).unwrap();
                        map = &cube_face.map;
                        match *destination_direction {
                            Direction::Down => {
                                next_col = face_width as i16 - next_row - 1;
                                next_row = face_width as i16 - 1;
                            }
                            Direction::Up => {
                                next_col = next_row;
                                next_row = 0;
                            }
                            Direction::Left => {
                                next_col = 0;
                                next_row = face_width as i16 - next_row - 1;
                            }
                            Direction::Right => {
                                next_col = face_width as i16 - 1;
                            }
                        }
                        direction = destination_direction.opposite().to_tuple();
                    }
                    if next_col >= face_width as i16 {
                        // right
                        let (destination_id, destination_direction) = cube_face.connections.get(&Direction::Right).unwrap();
                        cube_id = *destination_id;
                        cube_face = cube_faces.get(&cube_id).unwrap();
                        map = &cube_face.map;
                        match *destination_direction {
                            Direction::Down => {
                                next_col = next_row;
                                next_row = face_width as i16 - 1;
                            }
                            Direction::Up => {
                                next_col = face_width as i16 - next_row - 1;
                                next_row = 0;
                            }
                            Direction::Left => {
                                next_col = 0;
                            }
                            Direction::Right => {
                                next_col = face_width as i16 - 1;
                                next_row = face_width as i16 - next_row - 1;
                            }
                        }
                        direction = destination_direction.opposite().to_tuple();
                    }

                    let char = *map.get(&(next_row as u16, next_col as u16)).unwrap();
                    if char == '#' {
                        break;
                    }
                    if char == '.' {
                        new_position.position.row = next_row as u16;
                        new_position.position.col = next_col as u16;
                        new_position.position.direction = direction;
                        new_position.id = cube_id;
                    }
                }
                new_position
            }
            Instruction::TurnLeft => {
                let mut new_position = self.clone();
                new_position.position.rotate_left();
                new_position
            }
            Instruction::TurnRight => {
                let mut new_position = self.clone();
                new_position.position.rotate_right();
                new_position
            }
            
        }
    }
}

fn part2(map: &HashMap<(u16, u16), char>, instructions: &Vec<Instruction>) {
    let map_width = *map.keys().map(|(row, _)| row).max().unwrap() + 1;
    let map_height = *map.keys().map(|(_, col)| col).max().unwrap() + 1;
    let map_area = map_width * map_height;
    let face_width = (map_area as f64 / 12.0).sqrt() as u16;

    let mut squares: Vec<HashMap<(u16, u16), char>> = Vec::new();
    let square_rows = map_width / face_width;
    let square_cols = map_height / face_width;
    for i in 0..square_rows {
        for j in 0..square_cols {
            let mut square: HashMap<(u16, u16), char> = HashMap::new();
            for k in 0..face_width {
                for l in 0..face_width {
                    square.insert(
                        (k, l),
                        *map.get(&(i * face_width + k, j * face_width + l)).unwrap(),
                    );
                }
            }
            squares.push(square);
        }
    }

    let mut cube_faces: HashMap<u8, CubeFace> = HashMap::new();
    for i in 0..squares.len() {
        let row = i / square_cols as usize;
        let col = i % square_cols as usize;
        let square = squares.get(i).unwrap();
        if is_empty(&square) {
            continue;
        }
        let mut cube_face = CubeFace {
            id: i as u8,
            connections: HashMap::new(),
            map: square.clone(),
        };
        if col > 0 && !is_empty(&squares.get(i - 1).unwrap()) {
            cube_face
                .connections
                .insert(Direction::Left, (i as u8 - 1, Direction::Right));
        }
        if col < square_cols as usize - 1 && !is_empty(&squares.get(i + 1).unwrap()) {
            cube_face
                .connections
                .insert(Direction::Right, (i as u8 + 1, Direction::Left));
        }
        if row > 0 && !is_empty(&squares.get(i - square_cols as usize).unwrap()) {
            cube_face.connections.insert(
                Direction::Up,
                (i as u8 - square_cols as u8, Direction::Down),
            );
        }
        if row < square_rows as usize - 1
            && !is_empty(&squares.get(i + square_cols as usize).unwrap())
        {
            cube_face.connections.insert(
                Direction::Down,
                (i as u8 + square_cols as u8, Direction::Up),
            );
        }
        cube_faces.insert(i as u8, cube_face);
    }

    // fill in missing directions
    let mut new_cube_faces = cube_faces.clone();
    for face in cube_faces.clone() {
        let (id, cube_face) = face;
        let new_cube_face = add_connections(&cube_faces, cube_face.clone());
        new_cube_faces.insert(id, new_cube_face);
    }
    cube_faces = new_cube_faces;



    let lowest_id = *cube_faces.keys().min().unwrap();
    let mut position = CubePosition {
        id: lowest_id,
        position: Position {
            row: 0,
            col: 0,
            direction: Direction::Right.to_tuple()
        }
    };


    for instruction in instructions {
        position = position.do_move(instruction, face_width, position.id, &cube_faces);
        
    }

    let final_row = (position.id as usize / square_cols as usize) * face_width as usize + position.position.row  as usize + 1;
    let final_col = (position.id as usize % square_cols as usize) * face_width as usize + position.position.col  as usize + 1;
    let final_facing = match position.position.direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => panic!("Invalid direction"),
    };


    let final_password = 1000 * final_row + 4 * final_col + final_facing;

    println!("Day 22 part 2: {}", final_password);

}

fn is_empty(map: &HashMap<(u16, u16), char>) -> bool {
    for (_, char) in map {
        if *char != ' ' {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy)]
enum DirectionRelation {
    A,
    B,
    mA,
    mB,
}

impl DirectionRelation {
    fn to_direction(&self, a : Direction, b: Direction) -> Direction {
        match self {
            DirectionRelation::A => a,
            DirectionRelation::B => b,
            DirectionRelation::mA => a.opposite(),
            DirectionRelation::mB => b.opposite(),
        }
    }
}

fn add_connections(cube_faces: &HashMap<u8, CubeFace>, cube_face : CubeFace) -> CubeFace {
    let mut new_cube_face = cube_face.clone();
    let mut missing_directions: Vec<Direction> = Vec::new();
    if !cube_face.connections.contains_key(&Direction::Up) {
        missing_directions.push(Direction::Up);
    }
    if !cube_face.connections.contains_key(&Direction::Down) {
        missing_directions.push(Direction::Down);
    }
    if !cube_face.connections.contains_key(&Direction::Left) {
        missing_directions.push(Direction::Left);
    }
    if !cube_face.connections.contains_key(&Direction::Right) {
        missing_directions.push(Direction::Right);
    }


    // A -> B A -> -B
    // A -> -A B B -> A
    // A -> B B A -> A
    // A -> -A -A B -A -> B
    // A -> B -A B B -> B
    // A -> -A B -A -A B -> -A
    // A -> B -A -A B -A -> -A

    let mut paths : Vec<(Vec<DirectionRelation>, DirectionRelation)> = Vec::new();
    let a = DirectionRelation::A;
    let b = DirectionRelation::B;
    let ma = DirectionRelation::mA;
    let mb = DirectionRelation::mB;

    paths.push((vec![b, a], mb));
    paths.push((vec![ma, b, b], a));
    paths.push((vec![b, b, a], a));
    paths.push((vec![ma, ma, b, ma], b));
    paths.push((vec![b, ma, b, b], b));
    paths.push((vec![ma, b, ma, ma, b], ma));
    paths.push((vec![b, ma, ma, b, ma], ma));

    for a in missing_directions {
        let perpendicular_directions = a.perpendicular();
        for b in perpendicular_directions {
            for path in &paths {
                let (steps, destination) = path;
                let destination_direction = destination.to_direction(a, b);
                let mut can_follow_path = true;
                let mut destination_cube = cube_face.clone();
                for step in steps {
                    let step_direction = step.to_direction(a,b);
                    if !destination_cube.connections.contains_key(&step_direction) {
                        can_follow_path = false;
                        break;
                    }
                    let destination_id = &destination_cube.connections.get(&step_direction).unwrap().0;
                    destination_cube = cube_faces.get(destination_id).unwrap().clone();
                }
                if !can_follow_path {
                    continue;
                }
                new_cube_face.connections.insert(a, (destination_cube.id, destination_direction));
            }
        }
    }

    new_cube_face
}


