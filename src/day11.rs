use std::collections::HashMap;

use crate::input::read_input;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: char,
    left_operand: Option<usize>,
    right_operand: Option<usize>,
    divisor: usize,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

pub fn part1() {
    
    let mut monkeys = get_monkeys_from_input();

    for _ in 0..20 {
        run_round(&mut monkeys, 3);
    }

    // get top 2 monkeys sorted by inspection count
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    let monkey_business = monkeys[0].inspection_count * monkeys[1].inspection_count;

    println!("Day 11 Part 1: {}", monkey_business);
}

fn get_monkeys_from_input() -> Vec<Monkey> {
    let lines = read_input("day11");

    let mut monkeys : Vec<Monkey> = Vec::new();

    // group lines into groups of 7
    let line_groups = lines.chunks(7);
    for group in line_groups {
        let items_line: &String = &group[1];
        let operation_line: &String  = &group[2];
        let test_line: &String  = &group[3];
        let true_target_line: &String  = &group[4];
        let false_target_line: &String  = &group[5];

        let items = items_line.split(": ").nth(1).unwrap().split(", ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let mut operation_parts = operation_line.split(" = ").nth(1).unwrap().split(" ");
        let left_operand = operation_parts.nth(0).unwrap().parse::<usize>().ok();
        let operation = operation_parts.nth(0).unwrap().chars().nth(0).unwrap();
        let right_operand = operation_parts.nth(0).unwrap().parse::<usize>().ok();

        let divisor = test_line.split(" by ").nth(1).unwrap().parse::<usize>().unwrap();
        let true_target = true_target_line.split("throw to monkey ").nth(1).unwrap().parse::<usize>().unwrap();
        let false_target = false_target_line.split("throw to monkey ").nth(1).unwrap().parse::<usize>().unwrap();

        let monkey = Monkey {
            items: items,
            operation: operation,
            left_operand: left_operand,
            right_operand: right_operand,
            divisor: divisor,
            true_target: true_target,
            false_target: false_target,
            inspection_count: 0,
        };
        monkeys.push(monkey);
    } 
    monkeys
}

fn run_round(monkeys : &mut Vec<Monkey>, worry_division : usize ) {
    let mut new_items : HashMap<usize, Vec<usize>> = HashMap::new();
    let mut common_divisor = 1;
    for monkey in monkeys.iter() {
        common_divisor *= monkey.divisor;
    }

    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let items = &mut monkey.items;
        let items_to_add = new_items.remove(&i);
        if items_to_add.is_some() {
            items.extend(items_to_add.unwrap());
        }
        let left_operand = monkey.left_operand;
        let right_operand = monkey.right_operand;
        let operation = monkey.operation;
        let divisor = monkey.divisor;
        let true_target = monkey.true_target;
        let false_target = monkey.false_target;
        while items.len() > 0 {
            let item = items.remove(0);
            let left = match left_operand {
                Some(operand) => operand,
                None => item,
            };
            let right = match right_operand {
                Some(operand) => operand,
                None => item,
            };
            let mut new_item = match operation {
                '+' => left + right % common_divisor,
                '-' => left - right % common_divisor,
                '*' => left * right % common_divisor,
                '/' => left / right % common_divisor,
                _ => panic!("Unknown operation {}", operation),
            };
            new_item /= worry_division;
            
            let test_result = new_item % divisor == 0;
            let target = if test_result { true_target } else { false_target };
            new_items.entry(target).or_insert(Vec::new()).push(new_item);
            monkey.inspection_count += 1;
        }
    }
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let items = &mut monkey.items;
        let items_to_add = new_items.remove(&i);
        if items_to_add.is_some() {
            items.extend(items_to_add.unwrap());
        }
    }
}

pub fn part2() {
    let mut monkeys = get_monkeys_from_input();

    for i in 0..10000 {
        run_round(&mut monkeys, 1);
    }

    // get top 2 monkeys sorted by inspection count
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    let monkey_business = monkeys[0].inspection_count * monkeys[1].inspection_count;

    println!("Day 11 Part 2: {}", monkey_business);
}