use crate::input::read_input;


pub fn part1() {
    let lines = read_input("day5");

    let mut stacks : Vec<Vec<char>> = Vec::new();
    let stack_count = lines[0].len() / 4 + 1;
    let max_size = 8;
    
    for i in 0..max_size {
        for j in 0..stack_count {
            // add stack if it's not added yet
            if stacks.len() <= j {
                stacks.push(Vec::new());
            }
            let mut max_index = (j+1)*4;
            if max_index >= lines[0].len() {
                max_index = lines[0].len();
            }
            let segment = lines[max_size - i - 1].get(j*4..max_index).unwrap();
            let character = segment.chars().nth(1).unwrap();
            // add to stack if character is not blank
            if character != ' ' {
                stacks[j].push(character);
            }
        }
    }

    // execute instructions
    let instruction_lines = lines[10..].to_vec();

    for line in instruction_lines {
        // line is string in format move {count} from {stack_number_1} to {stack_number_2}
        let mut parts = line.split_whitespace();
        let count = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let from = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let to = parts.nth(1).unwrap().parse::<usize>().unwrap();

        for _ in 0..count {
            let character = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(character);
        }

    }

    // concatenate number at top of each stack
    let mut result = String::new();
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }

    println!("Day 5 Part 1: {}", result);
}

pub fn part2() {
    let lines = read_input("day5");

    let mut stacks : Vec<Vec<char>> = Vec::new();
    let stack_count = lines[0].len() / 4 + 1;
    let max_size = 8;
    
    for i in 0..max_size {
        for j in 0..stack_count {
            // add stack if it's not added yet
            if stacks.len() <= j {
                stacks.push(Vec::new());
            }
            let mut max_index = (j+1)*4;
            if max_index >= lines[0].len() {
                max_index = lines[0].len();
            }
            let segment = lines[max_size - i - 1].get(j*4..max_index).unwrap();
            let character = segment.chars().nth(1).unwrap();
            // add to stack if character is not blank
            if character != ' ' {
                stacks[j].push(character);
            }
        }
    }

    // execute instructions
    let instruction_lines = lines[10..].to_vec();

    for line in instruction_lines {
        // line is string in format move {count} from {stack_number_1} to {stack_number_2}
        let mut parts = line.split_whitespace();
        let count = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let from = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let to = parts.nth(1).unwrap().parse::<usize>().unwrap();

        // get top count elements from from stack in order
        let mut top_elements : Vec<char> = Vec::new();
        for i in 0..count {
            top_elements.push(stacks[from - 1][stacks[from - 1].len() - i - 1]);
        }

        // remove top count elements from from stack
        for _ in 0..count {
            stacks[from - 1].pop();
        }

        // add top count elements to to stack
        for i in 0..count {
            stacks[to - 1].push(top_elements[count - i - 1]);
        }

    }

    // concatenate number at top of each stack
    let mut result = String::new();
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }
    
    println!("Day 5 Part 2: {}", result);
}