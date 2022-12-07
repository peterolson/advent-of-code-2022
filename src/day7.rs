use crate::input::read_input;

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>
}

pub fn part1() {
    let directory = get_directory_from_input();

    let directories_under_size = get_directories_under_size(directory, 100000);

    let mut total_size = 0;
    for directory in directories_under_size {
        total_size += get_directory_size(directory);
    }

    println!("Day 7 Part 1: {}", total_size);
}

pub fn part2() {
    let directory = get_directory_from_input();

    let total_space = 70000000;
    let required_free_space = 30000000;
    let actual_free_space = total_space - get_directory_size(directory.clone());

    let (min_directory, min_size) = get_min_directory_over_size(directory.clone(), required_free_space - actual_free_space).unwrap();

    println!("Day 7 Part 2: {}", min_size);
}

fn get_directory_from_input() -> Directory {
    let lines = read_input("day7");

    let root_directory = Directory {
        name: String::from(""),
        files: Vec::new(),
        directories: Vec::new()
    };

    let mut input_output_pairs : Vec<(String, Vec<String>)> = Vec::new();
    for line in lines {
        if line.starts_with("$") {
            let input = line[2..].to_string();
            input_output_pairs.push((input, Vec::new()));
        } else {
            input_output_pairs.last_mut().unwrap().1.push(line);
        }
    }

    let mut directory_stack : Vec<Directory> = Vec::new();
    directory_stack.push(root_directory);
    for (input, output) in input_output_pairs {
        if input.starts_with("cd") {
            if input.ends_with("..") {
                pop_directory_stack(&mut directory_stack);
            } else if input.ends_with("/") {
                while directory_stack.len() > 1 {
                    pop_directory_stack(&mut directory_stack);
                }
            } else {
                let name = input[3..].to_string();
                let pwd = directory_stack.last().unwrap();
                for directory in &pwd.directories {
                    if directory.name == name {
                        directory_stack.push(directory.clone());
                        break;
                    }
                }
            }
        } 
        else if input.starts_with("ls") {
            for line in output {
                if line.starts_with("dir") {
                    let name = line[4..].to_string();
                    let mut already_exists = false;
                    let mut pwd = directory_stack.last().unwrap().clone();
                    for directory in &pwd.directories {
                        if directory.name == name {
                            already_exists = true;
                            break;
                        }
                    }
                    if !already_exists {
                        let new_directory = Directory {
                            name: name,
                            files: Vec::new(),
                            directories: Vec::new()
                        };
                        pwd.directories.push(new_directory);
                        directory_stack.pop();
                        directory_stack.push(pwd);
                    }
                }
                else {
                    let mut pwd = directory_stack.last().unwrap().clone();
                    // line is in format "size name"
                    let size = line[0..line.find(" ").unwrap()].parse::<u32>().unwrap();
                    let name = line[line.find(" ").unwrap() + 1..].to_string();
                    let mut already_exists = false;
                    for file in &pwd.files {
                        if file.name == name {
                            already_exists = true;
                            break;
                        }
                    }
                    if !already_exists {
                        let new_file = File {
                            name: name.clone(),
                            size: size,
                        };
                        pwd.files.push(new_file);
                        directory_stack.pop();
                        directory_stack.push(pwd);
                    }
                }
            }
        }
    }

    while directory_stack.len() > 1 {
        pop_directory_stack(&mut directory_stack);
    }
    directory_stack.first().unwrap().clone()
}

fn pop_directory_stack(directory_stack: &mut Vec<Directory>) {
    let last = directory_stack.last().unwrap().clone();
    directory_stack.pop();
    let mut new_last = directory_stack.last().unwrap().clone();
    // replace old subdirectory with new one
    for i in 0..new_last.directories.len() {
        if new_last.directories[i].name == last.name {
            new_last.directories.remove(i);
            new_last.directories.insert(i, last);
            break;
        }
    }
    directory_stack.pop();
    directory_stack.push(new_last);
}

fn get_directory_size(directory: Directory) -> u32 {
    let mut size = 0;
    for file in &directory.files {
        size += file.size;
    }
    for directory in directory.directories {
        size += get_directory_size(directory);
    }
    size
}

fn get_directories_under_size( directory: Directory, size: u32) -> Vec<Directory> {   
    let mut directories : Vec<Directory> = Vec::new();
    if get_directory_size(directory.clone()) <= size {
        directories.push(directory.clone());
    }
    let subdirectories = directory.clone().directories;
    for subdirectory in subdirectories {
        let mut subdirectories = get_directories_under_size(subdirectory.clone(), size);
        directories.append(&mut subdirectories);
    }
    directories
}

fn get_min_directory_over_size(directory: Directory, size: u32) -> Option<(Directory, u32)> {
    let mut min_directory = directory.clone();
    let mut min_size = get_directory_size(directory.clone());
    let subdirectories = directory.clone().directories;
    for subdirectory in subdirectories {
        let subdirectory_option = get_min_directory_over_size(subdirectory.clone(), size);
        if subdirectory_option.is_none() {
            continue;
        }
        let (subdirectory, subdirectory_min_size) = subdirectory_option.unwrap();
        if subdirectory_min_size < min_size && subdirectory_min_size > size {
            min_directory = subdirectory.clone();
            min_size = subdirectory_min_size;
        }
    }
    if min_size >= size {
        Some((min_directory, min_size))
    } else {
        None
    }
}