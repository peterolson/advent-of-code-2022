use crate::input::read_input;

pub fn part1() -> Vec<i32> {
    let lines = read_input("day10");

    let mut X = 1;
    let mut signal_strengths : Vec<i32> = Vec::new();
    let mut x_values : Vec<i32> = Vec::new();
    let mut cycle_number = 0;
    signal_strengths.push(X * cycle_number);
    x_values.push(X);

    for line in lines {
        cycle_number += 1;
        signal_strengths.push(X * cycle_number);
        x_values.push(X);
        // if line starts with "addx"
        if line.starts_with("addx") {
            // get the number after "addx "
            let number = line.split(" ").nth(1).unwrap();
            // add the number to X
            cycle_number += 1;
            signal_strengths.push(X * cycle_number);
            X += number.parse::<i32>().unwrap();
            x_values.push(X);
        }
    }

    let sum = signal_strengths[20] + signal_strengths[60] + signal_strengths[100] + signal_strengths[140] + signal_strengths[180] + signal_strengths[220];

    println!("Day 10 Part 1: {}", sum);

    return x_values;
}

pub fn part2() {
    let x_values = part1();
    let mut sprite_start : i32 = 0;
    let mut sprite_end : i32 = 3;

    let mut output = String::new();

    for i in 1..x_values.len() {
        let pos = ((i as i32) - 1) % 40;
        if pos >= sprite_start && pos < sprite_end {
            output.push('#');
        } else {
            output.push('.');
        }
        let x = x_values[i];
        sprite_start = x - 1;
        sprite_end = sprite_start + 3;
    }
    println!("Day 10 Part 2: ");
    // display 40 chars at a time
    for i in (0..output.len()).step_by(40) {
        let mut max_index = i + 40;
        if max_index > output.len() {
            max_index = output.len();
        }
        println!("{}", &output[i..max_index]);
    }

}