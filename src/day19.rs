use std::{collections::{VecDeque, HashSet, HashMap}, cmp::Ordering};

use crate::input::read_input;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn can_afford(&self, other: &Resources) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian && self.geode >= other.geode
    }

    fn spend(&self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

pub fn part1() {
    let lines = read_input("day19");

    let mut blueprints = Vec::new();

    for line in lines {
        println!("{}", line);
        let parts = line.split(": ").collect::<Vec<&str>>();
        // first part is format Blueprint 1
        let blueprint_id = parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let sentences = parts[1].split(".").collect::<Vec<&str>>();
        let ore_robot_cost = parse_sentence(sentences[0]);
        let clay_robot_cost = parse_sentence(sentences[1]);
        let obsidian_robot_cost = parse_sentence(sentences[2]);
        let geode_robot_cost = parse_sentence(sentences[3]);
        let blueprint = Blueprint {
            id: blueprint_id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        };
        blueprints.push(blueprint);
    }
    
    println!("{:?}", blueprints);

    let mut total_quality_level = 0;
    for blueprint in blueprints {
        let geode_count = count_open_geodes(&blueprint, 24);
        let quality_level = geode_count * blueprint.id;
        total_quality_level += quality_level;
    }

    println!("Day 19, part 1: {}", total_quality_level);
}

fn parse_sentence(sentence: &str) -> Resources {
    // example sentence: Each geode robot costs 3 ore and 12 obsidian
    let content = sentence.split("costs ").collect::<Vec<&str>>()[1];
    let parts = content.split(" and ").collect::<Vec<&str>>();
    let mut resources = Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    for part in parts {
        let part_parts = part.split(" ").collect::<Vec<&str>>();
        let amount = part_parts[0].parse::<usize>().unwrap();
        let resource = part_parts[1];
        match resource {
            "ore" => resources.ore = amount,
            "clay" => resources.clay = amount,
            "obsidian" => resources.obsidian = amount,
            _ => panic!("Unknown resource: {}", resource),
        }
    }
    resources
}

// Robot state includes the resources, minutes left, ore robots, clay robots, obsidian robots, geode robots
type RobotState = (Resources, usize, usize, usize, usize, usize);

fn count_open_geodes(blueprint : &Blueprint, minutes: usize) -> usize {
    let mut geodes_opened = 0;
    
    let mut robot_states : VecDeque<RobotState> = VecDeque::new();
    robot_states.push_back((Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    }, minutes, 1, 0, 0, 0));

    let ore_bottleneck = blueprint.ore_robot_cost.ore.max(blueprint.clay_robot_cost.ore).max(blueprint.obsidian_robot_cost.ore).max(blueprint.geode_robot_cost.ore);
    let clay_bottleneck = blueprint.ore_robot_cost.clay.max(blueprint.clay_robot_cost.clay).max(blueprint.obsidian_robot_cost.clay).max(blueprint.geode_robot_cost.clay);
    let obsidian_bottleneck = blueprint.ore_robot_cost.obsidian.max(blueprint.clay_robot_cost.obsidian).max(blueprint.obsidian_robot_cost.obsidian).max(blueprint.geode_robot_cost.obsidian);

    let mut iterations = 0;

    let mut best_states : HashMap<usize, RobotState> = HashMap::new();

    while robot_states.len() > 0 {
        let state = robot_states.pop_front().unwrap();
        let (resources, minutes_left, ore_robots, clay_robots, obsidian_robots, geode_robots) = state.clone();
        if !best_states.contains_key(&minutes_left){
            best_states.insert(minutes_left, state);
        } else {
            let best_state = best_states.get(&minutes_left).unwrap().clone();
            let comparison = compare_robot_state(&state, &best_state, blueprint);
            println!("{:?} vs {:?} = {:?}", state, best_state, comparison);
            if comparison == Ordering::Greater {
                best_states.insert(minutes_left, state);
            }
            if comparison == Ordering::Less {
                continue;
            }
        }
        iterations += 1;
        if iterations % 100000 == 0 {
            println!("Blueprint #{} : {} iterations ({}), {} minutes left {} ore, {} clay, {} obsidian, {} geode {:?}", 
            blueprint.id, iterations, robot_states.len(), minutes_left, obsidian_robots, clay_robots, obsidian_robots, geode_robots, resources);
        }
        if minutes_left == 0 {
            // no more time left, count the geodes
            if resources.geode > geodes_opened {
                geodes_opened = resources.geode;
            }
            continue;
        }
       // println!("{} minutes left, {} ore robots, {} clay robots, {} obsidian robots, {} geode robots, {:?}", minutes_left, ore_robots, clay_robots, obsidian_robots, geode_robots, resources);

        let next_round_resources = Resources {
            ore: resources.ore + ore_robots,
            clay: resources.clay + clay_robots,
            obsidian: resources.obsidian + obsidian_robots,
            geode: resources.geode + geode_robots,
        };

        
        // option for creating geode robot
        if resources.can_afford(&blueprint.geode_robot_cost) {
            let next_round_resources = next_round_resources.spend(&blueprint.geode_robot_cost);
            robot_states.push_back((next_round_resources, minutes_left - 1, ore_robots, clay_robots, obsidian_robots, geode_robots + 1));
        }
        
        // option for creating obsidian robot
        if resources.can_afford(&blueprint.obsidian_robot_cost) && obsidian_robots < obsidian_bottleneck {
            let next_round_resources = next_round_resources.spend(&blueprint.obsidian_robot_cost);
            robot_states.push_back((next_round_resources, minutes_left - 1, ore_robots, clay_robots, obsidian_robots + 1, geode_robots));
        }

        
        // option for creating clay robot
        if resources.can_afford(&blueprint.clay_robot_cost) && clay_robots < clay_bottleneck {
            let next_round_resources = next_round_resources.spend(&blueprint.clay_robot_cost);
            robot_states.push_back((next_round_resources, minutes_left - 1, ore_robots, clay_robots + 1, obsidian_robots, geode_robots));
        }
        
        // option for creating ore robot
        if resources.can_afford(&blueprint.ore_robot_cost) && ore_robots < ore_bottleneck {
            let next_round_resources = next_round_resources.spend(&blueprint.ore_robot_cost);
            robot_states.push_back((next_round_resources, minutes_left - 1, ore_robots + 1, clay_robots, obsidian_robots, geode_robots));
        }

        // option for not creating any robots
        robot_states.push_back((next_round_resources, minutes_left - 1, ore_robots, clay_robots, obsidian_robots, geode_robots));
    }

    geodes_opened
}

fn compare_robot_state(robot_state_1 : &RobotState, robot_state_2: &RobotState, blueprint : &Blueprint) -> Ordering {
    let (resources_1, minutes_left, ore_robots_1, clay_robots_1, obsidian_robots_1, geode_robots_1) = robot_state_1;
    let (resources_2, minutes_left, ore_robots_2, clay_robots_2, obsidian_robots_2, geode_robots_2) = robot_state_2;
    let total_geodes_1 = resources_1.geode + geode_robots_1 * minutes_left;
    let total_geodes_2 = resources_2.geode + geode_robots_2 * minutes_left;
    if total_geodes_1 != total_geodes_2 {
        return total_geodes_1.cmp(&total_geodes_2);
    }
    let total_obsidian_1 = resources_1.obsidian + obsidian_robots_1 * minutes_left;
    let total_obsidian_2 = resources_2.obsidian + obsidian_robots_2 * minutes_left;
    if total_obsidian_1 != total_obsidian_2 {
        return total_obsidian_1.cmp(&total_obsidian_2);
    }
    let total_clay_1 = resources_1.clay + clay_robots_1 * minutes_left;
    let total_clay_2 = resources_2.clay + clay_robots_2 * minutes_left;
    if total_clay_1 != total_clay_2 {
        return total_clay_1.cmp(&total_clay_2);
    }

    let total_ore_1 = resources_1.ore + ore_robots_1 * minutes_left;
    let total_ore_2 = resources_2.ore + ore_robots_2 * minutes_left;

    return total_ore_1.cmp(&total_ore_2);
}