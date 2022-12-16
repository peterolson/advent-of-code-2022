use std::collections::{HashMap, VecDeque};

use crate::input::read_input;

type Label = (char, char);

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    targets: Vec<Label>,
}

pub fn part1() {
    let lines = read_input("day16");

    let mut valves_map: HashMap<Label, Valve> = HashMap::new();
    for line in lines {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let label = (
            words[1].chars().nth(0).unwrap(),
            words[1].chars().nth(1).unwrap(),
        );
        let flow_rate_str = words[4]; // rate=0;
        let flow_rate = flow_rate_str[5..flow_rate_str.len() - 1]
            .parse::<usize>()
            .unwrap();
        let mut targets = Vec::new();
        for i in 9..words.len() {
            let target = (
                words[i].chars().nth(0).unwrap(),
                words[i].chars().nth(1).unwrap(),
            );
            targets.push(target);
        }
        let valve = Valve { flow_rate, targets };
        valves_map.insert(label, valve);
    }

    let non_zero_valves = valves_map
        .keys()
        .filter(|k| {
            let valve = valves_map.get(k).unwrap();
            valve.flow_rate > 0
        })
        .collect::<Vec<&Label>>();

    let mut relevant_labels = non_zero_valves.clone();
    relevant_labels.push(&('A', 'A'));
    let mut distances: HashMap<(Label, Label), usize> = HashMap::new();
    for label1 in &relevant_labels {
        for label2 in &relevant_labels {
            distance_between_labels(label1, label2, &valves_map, Vec::new(), &mut distances);
        }
    }

    let mut search_queue: VecDeque<(Vec<Label>, usize, usize)> = VecDeque::new();
    search_queue.push_back((Vec::new(), 30, 0));
    let mut max_pressure = 0;
    while search_queue.len() > 0 {
        let (current_path, minutes_remaining, total_pressure) = search_queue.pop_front().unwrap();
        let mut current_label: Label = ('A', 'A');
        let mut can_move = false;
        if current_path.len() > 0 {
            current_label = current_path[current_path.len() - 1];
        }
        for target in &non_zero_valves {
            if current_path.contains(target) {
                continue;
            }
            let distance = *distances.get(&(current_label, **target)).unwrap();
            if distance + 1 >= minutes_remaining {
                continue;
            }
            can_move = true;
            let mut new_path = current_path.clone();
            new_path.push(**target);
            let active_minutes = minutes_remaining - distance - 1;
            let total_pressure =
                total_pressure + valves_map.get(target).unwrap().flow_rate * active_minutes;
            search_queue.push_back((new_path, active_minutes, total_pressure));
        }
        if !can_move {
            if total_pressure > max_pressure {
                max_pressure = total_pressure;
            }
        }
    }

    println!("Day 16 Part 1: {}", max_pressure);

    search_queue.clear();
    search_queue.push_back((Vec::new(), 26, 0));
    let mut pressure_dict: HashMap<Vec<Label>, usize> = HashMap::new();

    while search_queue.len() > 0 {
        let (current_path, minutes_remaining, total_pressure) = search_queue.pop_front().unwrap();
        let mut current_label: Label = ('A', 'A');
        if current_path.len() > 0 {
            current_label = current_path[current_path.len() - 1];
        }
        for target in &non_zero_valves {
            if current_path.contains(target) {
                continue;
            }
            let distance = *distances.get(&(current_label, **target)).unwrap();
            if distance + 1 >= minutes_remaining {
                continue;
            }
            let mut new_path = current_path.clone();
            new_path.push(**target);
            let active_minutes = minutes_remaining - distance - 1;
            let total_pressure =
                total_pressure + valves_map.get(target).unwrap().flow_rate * active_minutes;
            search_queue.push_back((new_path.clone(), active_minutes, total_pressure));
            new_path.sort();
            if pressure_dict.contains_key(&new_path) {
                let existing_pressure = pressure_dict.get(&new_path).unwrap();
                if total_pressure > *existing_pressure {
                    pressure_dict.insert(new_path, total_pressure);
                }
            } else {
                pressure_dict.insert(new_path, total_pressure);
            }
        }
    }

    max_pressure = 0;

    let pressure_dict_keys = pressure_dict.keys().collect::<Vec<&Vec<Label>>>();
    for i in 0..pressure_dict_keys.len() {
        let key = pressure_dict_keys[i];
        let person_path = key;
        let person_pressure = pressure_dict.get(key).unwrap();
        for j in i + 1..pressure_dict_keys.len() {
            let key = pressure_dict_keys[j];
            let elephant_path = key;
            let elephant_pressure = pressure_dict.get(key).unwrap();
            let has_common_path = person_path.iter().any(|x| elephant_path.contains(x));
            if has_common_path {
                continue;
            }
            let total_pressure = person_pressure + elephant_pressure;
            if total_pressure > max_pressure {
                max_pressure = total_pressure;
            }
        }
    }

    println!("Day 16 Part 2: {}", max_pressure);
}

fn distance_between_labels(
    label1: &Label,
    label2: &Label,
    valves_map: &HashMap<Label, Valve>,
    covered: Vec<Label>,
    distances: &mut HashMap<(Label, Label), usize>,
) -> usize {
    if distances.contains_key(&(label1.clone(), label2.clone())) {
        return *distances.get(&(label1.clone(), label2.clone())).unwrap();
    }
    if label1 == label2 {
        distances.insert((label1.clone(), label2.clone()), 0);
        return 0;
    }
    let targets = valves_map.get(label1).unwrap().targets.clone();
    if targets.contains(label2) {
        distances.insert((label1.clone(), label2.clone()), 1);
        return 1;
    }
    let mut min_distance = usize::MAX - 1;
    let mut new_covered = covered.clone();
    new_covered.push(label1.clone());
    for target in targets {
        if covered.contains(&target) {
            continue;
        }
        let distance =
            distance_between_labels(&target, label2, valves_map, new_covered.clone(), distances);
        if distance < min_distance {
            min_distance = distance;
        }
    }
    if covered.len() == 0 {
        distances.insert((label1.clone(), label2.clone()), min_distance + 1);
    }
    return min_distance + 1;
}
