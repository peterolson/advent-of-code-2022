pub fn read_input(file_name : &str) -> Vec<String> {
    // read input from file
    let input = std::fs::read_to_string(format!("input/{}.txt", file_name)).unwrap();
    // split input into lines
    let lines: Vec<&str> = input.lines().collect();
    // convert lines to strings
    let lines: Vec<String> = lines.iter().map(|&s| s.to_string()).collect();
    lines
}