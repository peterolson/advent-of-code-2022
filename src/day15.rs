use std::collections::HashMap;

use crate::input::read_input;

#[derive(Debug, Clone)]
struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64,
    distance: i64
}

#[derive(Debug, Clone)]
struct Interval {
    start: i64,
    end: i64
}

impl Interval {
    fn intersects(&self, other: &Interval) -> bool {
        if self.start <= other.start && self.end >= other.start {
            return true;
        }
        if self.start <= other.end && self.end >= other.end {
            return true;
        }
        if other.start <= self.start && other.end >= self.start {
            return true;
        }
        if other.start <= self.end && other.end >= self.end {
            return true;
        }
        false
    }

    fn join(&self, other: &Interval) -> Vec<Interval> {
        if self.intersects(other) {
            let mut result = Vec::new();
            result.push(Interval { 
                start: std::cmp::min(self.start, other.start), 
                end: std::cmp::max(self.end, other.end) 
             });
            return result;
        }
        vec![self.clone(), other.clone()]
    }

    fn add_to_list(&self, list: &mut Vec<Interval>) {
        let mut new_list = Vec::new();
        let mut self_interval = self.clone();
        for interval in &list.clone() {
            if self_interval.intersects(interval) {
                self_interval = self_interval.join(interval)[0].clone();
            } else {
                new_list.push(interval.clone());
            }
        }
        new_list.push(self_interval);
        list.clear();
        list.append(&mut new_list);
    }

    fn width(&self) -> i64 {
        self.end - self.start - 1
    }

    fn constrain(&self, list: &mut Vec<Interval>) {
        let min = self.start;
        let max = self.end;
        let mut new_list = Vec::new();
        for interval in &list.clone() {
            let mut interval = interval.clone();
            if interval.start < min {
                interval.start = min;
            }
            if interval.end > max {
                interval.end = max;
            }
            if interval.start < interval.end {
                new_list.push(interval.clone());
            }
        }
        list.clear();
        list.append(&mut new_list);
    }
}


pub fn part1_and_2() {
    let lines = read_input("day15");
    let mut sensors: Vec<Sensor> = Vec::new();

    let mut min_x = i64::max_value();
    let mut max_x = i64::min_value();

    for line in lines {
        let halves = line.split(": ").collect::<Vec<&str>>();
        let left = halves[0];
        let right = halves[1];
        // left format is "Sensor at x=2, y=18"
        let x = left.split("x=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
        let y = left.split("y=").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
        // right format is "closest beacon is at x=-2, y=15"
        let beacon_x = right.split("x=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
        let beacon_y = right.split("y=").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
        let distance = ((x - beacon_x).abs() + (y - beacon_y).abs());
        if x - distance < min_x {
            min_x = x - distance;
        }
        if x + distance > max_x {
            max_x = x + distance;
        }
        sensors.push(Sensor { x, y, beacon_x, beacon_y, distance });
    }

    let mut rows : HashMap<i64, Vec<Interval>> = HashMap::new();
    for sensor in &sensors {
        populate_intervals(&mut rows, sensor);
    }

    let intervals = rows.get(&2000000).unwrap();
    let mut spaces_in_range = 0;
    for interval in intervals {
        spaces_in_range += interval.width();
    }

    println!("Day 15 Part 1: {}", spaces_in_range);

    let search_space = Interval { start: 0, end: 4000000 + 1 };
    for y in search_space.start..search_space.end {
        if y % 100000 == 0 {
            println!("y = {}", y);
        }
        let mut intervals = rows.get(&y).unwrap().clone();
        search_space.constrain(&mut intervals);
        if intervals.len() > 1 {
            intervals.sort_by(|a, b| a.start.cmp(&b.start));
            println!("Found multiple intervals at y={}", y);
            for interval in &intervals {
                println!("Interval: {:?}", interval);
            }
            let x = intervals[0].end;
            println!("Beacon at x={}, y={}", x, y);
            let tuning_frequency = x * 4000000 + y;
            println!("Day 15 Part 2: {}", tuning_frequency);
            break;;
        }
    }
}

fn populate_intervals(rows: &mut HashMap<i64, Vec<Interval>>, sensor: &Sensor) {
    println!("Populating intervals for {:?}", sensor);
    for y in (sensor.y - sensor.distance)..(sensor.y + sensor.distance + 1) {
        let width = (sensor.distance - (y - sensor.y).abs()) as i64;
        let interval = Interval { start: sensor.x - width, end: sensor.x + width + 1 };
        let mut intervals = rows.entry(y).or_insert(Vec::new());
        interval.add_to_list(&mut intervals);
    }
}