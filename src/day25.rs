use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day25");
    let mut total = 0;
    for line in lines {
        let decimal = snafu_to_decimal(&line);
        total += decimal;
    }

    println!("Day 25 Part 1: {}", total);

    let snafu = decimal_to_snafu(total);

    println!("Day 25 Part 1: {}", snafu);
}

fn snafu_to_decimal(snafu: &str) -> isize {
    let chars = snafu.chars().collect::<Vec<char>>();
    let mut place = 1;
    let mut total = 0;
    for i in 0..chars.len() {
        let c = chars[chars.len() - i - 1];
        match c {
            '0' => {}
            '1' => total += place,
            '2' => total += place * 2,
            '-' => total += place * -1,
            '=' => total += place * -2,
            _ => panic!("Invalid snafu character: {}", c),
        }
        place *= 5;
    }
    total
}

fn decimal_to_snafu(decimal: isize) -> String {
    let mut snafu = String::new();
    if decimal < 3 {
        return decimal.to_string();
    }

    let mut digits = 1;
    let mut place = 1;
    let mut total = 2 * place;
    while total < decimal {
        digits += 1;
        place *= 5;
        total += 2 * place;
    }

    let max_remaining = total - 2 * place;

    let digit = (decimal + max_remaining) / place;

    let remainder = decimal - (digit * place);
    let mut remainder_string = String::new();

    if remainder >= 0 {
        remainder_string.push_str(&decimal_to_snafu(remainder));
    } else {
        let neg_remainder = remainder * -1;
        let result = &decimal_to_snafu(neg_remainder);
        // replace 2 with = and 1 with -, and vice-versa
        let mut replaced = String::new();
        for c in result.chars() {
            match c {
                '2' => replaced.push('='),
                '=' => replaced.push('2'),
                '1' => replaced.push('-'),
                '-' => replaced.push('1'),
                _ => replaced.push(c),
            }
        }
        remainder_string.push_str(&replaced);
    }

    snafu.push_str(&digit.to_string());
    for _ in 0..(digits - remainder_string.len() - 1) {
        snafu.push('0');
    }
    snafu.push_str(&remainder_string);
    
    snafu
}