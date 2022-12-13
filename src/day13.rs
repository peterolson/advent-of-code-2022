use json::JsonValue;

use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day13");

    let mut packet_pairs : Vec<(JsonValue, JsonValue)> = Vec::new();

    // go through lines in chunks of 3
    for i in (0..lines.len()).step_by(3) {
        let left = json::parse(&lines[i]).unwrap();
        let right = json::parse(&lines[i + 1]).unwrap();
        packet_pairs.push((left, right));
        
    }

    let mut index_sum = 0;
    
    for i in 0..packet_pairs.len() {
        let left = packet_pairs[i].0.clone();
        let right = packet_pairs[i].1.clone();
        let index = i + 1;
        if compare(left, right) <= 0 {
            index_sum += index;
        }
    }

    println!("Day 13 Part 1: {}", index_sum);
}

fn compare(left : JsonValue, right: JsonValue) -> i8 {
    if left.is_number() && right.is_number() {
        let left_number = left.as_i32().unwrap();
        let right_number = right.as_i32().unwrap();
        if left_number < right_number {
            return -1;
        }
        if left_number > right_number {
            return 1;
        }
        return 0;
    }
    if left.is_array() && right.is_array() {
        let mut left_array = left.members();
        let mut right_array = right.members();
        let left_len = left_array.len();
        let right_len = right_array.len();
        if left_len == 0 && right_len == 0 {
            return 0;
        }
        if left_len == 0 {
            return -1;
        }
        if right_len == 0 {
            return 1;
        }
        let left_item = left_array.next().unwrap();
        let right_item = right_array.next().unwrap();
        let compare_result = compare(left_item.clone(), right_item.clone());
        if compare_result != 0 {
            return compare_result;
        }
        while left_array.len() > 0 {
            if right_array.len() == 0 {
                return 1;
            }
            let left_item = left_array.next().unwrap();
            let right_item = right_array.next().unwrap();
            let compare_result = compare(left_item.clone(), right_item.clone());
            if compare_result != 0 {
                return compare_result;
            }
        }
        return -1;
    }
    if left.is_number() && right.is_array() {
        let new_left = JsonValue::Array(vec![left]);
        return compare(new_left, right);
    }
    let new_right = JsonValue::Array(vec![right]);
    return compare(left, new_right);
}

pub fn part2() {
    let lines = read_input("day13");

    let mut packets : Vec<JsonValue> = Vec::new();

    // go through lines in chunks of 3
    for i in (0..lines.len()).step_by(3) {
        let left = json::parse(&lines[i]).unwrap();
        let right = json::parse(&lines[i + 1]).unwrap();
        packets.push(left);
        packets.push(right);
    }
    let divider_1 = json::parse("[[2]]").unwrap();
    let divider_2 = json::parse("[[6]]").unwrap();
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort_by(|a, b| {
        let compare_result = compare(a.clone(), b.clone());
        if compare_result < 0 {
            return std::cmp::Ordering::Less;
        }
        if compare_result > 0 {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    });

    let mut decoder_key = 1;

    for i in 0..packets.len() {
        if packets[i] == divider_1 {
            decoder_key *= i + 1;
        }
        if packets[i] == divider_2 {
            decoder_key *= i + 1;
        }
    }

    println!("Day 13 Part 2: {}", decoder_key);
}