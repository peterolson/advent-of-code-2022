use std::collections::VecDeque;

use crate::input::read_input;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn spend(&self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }

    fn accumulate(
        &self,
        minutes: usize,
        ore_robots: usize,
        clay_robots: usize,
        obsidian_robots: usize,
        geode_robots: usize,
    ) -> Resources {
        Resources {
            ore: self.ore + ore_robots * minutes,
            clay: self.clay + clay_robots * minutes,
            obsidian: self.obsidian + obsidian_robots * minutes,
            geode: self.geode + geode_robots * minutes,
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
        let parts = line.split(": ").collect::<Vec<&str>>();
        // first part is format Blueprint 1
        let blueprint_id = parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
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

    let mut total_quality_level = 0;
    for blueprint in &blueprints {
        let geode_count = count_open_geodes(blueprint, 24);
        let quality_level = geode_count * blueprint.id;
        total_quality_level += quality_level;
    }

    println!("Day 19, part 1: {}", total_quality_level);

    let mut geodes_product = 1;
    for i in 0..3 {
        let blueprint = &blueprints[i];
        let geode_count = count_open_geodes(blueprint, 32);
        println!("Blueprint #{}: {} geodes", blueprint.id, geode_count);
        geodes_product *= geode_count;
    }

    println!("Day 19, part 2: {}", geodes_product);
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

fn count_open_geodes(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut geodes_opened = 0;

    let mut robot_states: VecDeque<RobotState> = VecDeque::new();
    robot_states.push_back((
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        minutes,
        1,
        0,
        0,
        0,
    ));

    let ore_bottleneck = blueprint
        .ore_robot_cost
        .ore
        .max(blueprint.clay_robot_cost.ore)
        .max(blueprint.obsidian_robot_cost.ore)
        .max(blueprint.geode_robot_cost.ore);
    let clay_bottleneck = blueprint
        .ore_robot_cost
        .clay
        .max(blueprint.clay_robot_cost.clay)
        .max(blueprint.obsidian_robot_cost.clay)
        .max(blueprint.geode_robot_cost.clay);
    let obsidian_bottleneck = blueprint
        .ore_robot_cost
        .obsidian
        .max(blueprint.clay_robot_cost.obsidian)
        .max(blueprint.obsidian_robot_cost.obsidian)
        .max(blueprint.geode_robot_cost.obsidian);

    while robot_states.len() > 0 {
        let state = robot_states.pop_front().unwrap();
        let (resources, minutes_left, ore_robots, clay_robots, obsidian_robots, geode_robots) =
            state.clone();

        if ore_robots < ore_bottleneck {
            let cost = &blueprint.ore_robot_cost;
            let time_to_build = div_ceil(cost.ore.saturating_sub(resources.ore), ore_robots);
            if time_to_build >= minutes_left {
                let geodes = minutes_left * geode_robots + resources.geode;
                if geodes > geodes_opened {
                    geodes_opened = geodes;
                }
                continue;
            }
            robot_states.push_back((
                resources
                    .accumulate(
                        time_to_build + 1,
                        ore_robots,
                        clay_robots,
                        obsidian_robots,
                        geode_robots,
                    )
                    .spend(&blueprint.ore_robot_cost),
                minutes_left - time_to_build - 1,
                ore_robots + 1,
                clay_robots,
                obsidian_robots,
                geode_robots,
            ));
        }

        if clay_robots < clay_bottleneck {
            let cost = &blueprint.clay_robot_cost;
            let time_to_build = div_ceil(cost.ore.saturating_sub(resources.ore), ore_robots);
            if time_to_build >= minutes_left {
                let geodes = minutes_left * geode_robots + resources.geode;
                if geodes > geodes_opened {
                    geodes_opened = geodes;
                }
                continue;
            }
            robot_states.push_back((
                resources
                    .accumulate(
                        time_to_build + 1,
                        ore_robots,
                        clay_robots,
                        obsidian_robots,
                        geode_robots,
                    )
                    .spend(&blueprint.clay_robot_cost),
                minutes_left - time_to_build - 1,
                ore_robots,
                clay_robots + 1,
                obsidian_robots,
                geode_robots,
            ));
        }

        if clay_robots > 0 && obsidian_robots < obsidian_bottleneck {
            let cost = &blueprint.obsidian_robot_cost;
            let ore_time_to_build = div_ceil(cost.ore.saturating_sub(resources.ore), ore_robots);
            let clay_time_to_build =
                div_ceil(cost.clay.saturating_sub(resources.clay), clay_robots);
            let time_to_build = ore_time_to_build.max(clay_time_to_build);
            if time_to_build >= minutes_left {
                let geodes = minutes_left * geode_robots + resources.geode;
                if geodes > geodes_opened {
                    geodes_opened = geodes;
                }
                continue;
            }
            robot_states.push_back((
                resources
                    .accumulate(
                        time_to_build + 1,
                        ore_robots,
                        clay_robots,
                        obsidian_robots,
                        geode_robots,
                    )
                    .spend(&blueprint.obsidian_robot_cost),
                minutes_left - time_to_build - 1,
                ore_robots,
                clay_robots,
                obsidian_robots + 1,
                geode_robots,
            ));
        }

        if obsidian_robots > 0 {
            let cost = &blueprint.geode_robot_cost;
            let ore_time_to_build = div_ceil(cost.ore.saturating_sub(resources.ore), ore_robots);
            let obsidian_time_to_build = div_ceil(
                cost.obsidian.saturating_sub(resources.obsidian),
                obsidian_robots,
            );
            let time_to_build = ore_time_to_build.max(obsidian_time_to_build);
            if time_to_build >= minutes_left {
                let geodes = minutes_left * geode_robots + resources.geode;
                if geodes > geodes_opened {
                    geodes_opened = geodes;
                }
                continue;
            }
            robot_states.push_back((
                resources
                    .accumulate(
                        time_to_build + 1,
                        ore_robots,
                        clay_robots,
                        obsidian_robots,
                        geode_robots,
                    )
                    .spend(&blueprint.geode_robot_cost),
                minutes_left - time_to_build - 1,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots + 1,
            ));
        }
    }

    geodes_opened
}

fn div_ceil(a: usize, b: usize) -> usize {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}
