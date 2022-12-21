use std::collections::HashMap;

use crate::input::read_input;

#[derive(Debug, Clone)]
struct Monkey {
    left_operand: Option<String>,
    right_operand: Option<String>,
    operator: Option<String>,
    value: Option<isize>,
}

pub fn part1() {
    let lines = read_input("day21");
    let mut monkeys = HashMap::new();
    for line in lines {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let name = parts[0];
        let value = parts[1];
        let value_parts = value.split(" ").collect::<Vec<&str>>();
        if value_parts.len() > 2 {
            let left_operand = Some(value_parts[0].to_string());
            let operator = Some(value_parts[1].to_string());
            let right_operand = Some(value_parts[2].to_string());
            monkeys.insert(
                name.to_string(),
                Monkey {
                    left_operand,
                    right_operand,
                    operator,
                    value: None,
                },
            );
        } else {
            let value = value_parts[0].parse::<isize>().unwrap();
            monkeys.insert(
                name.to_string(),
                Monkey {
                    left_operand: None,
                    right_operand: None,
                    operator: None,
                    value: Some(value),
                },
            );
        }
    }

    let root_number = get_number(&monkeys, "root");

    println!("Day 21 Part 1: {}", root_number);

    let monkey_expression = to_expression(&monkeys, "root");
    let lhs = monkey_expression.get_lhs();
    let rhs = monkey_expression.get_rhs();

    let solution = solve_equation(lhs, rhs);

    println!("Day 21 Part 2: {}", solution);
}

fn get_number(monkeys: &HashMap<String, Monkey>, name: &str) -> isize {
    let monkey = monkeys.get(name).unwrap().clone();
    let value = monkey.value;
    if value.is_some() {
        return monkey.value.unwrap();
    }
    let left_operand = &monkey.left_operand.unwrap().clone();
    let right_operand = &monkey.right_operand.unwrap().clone();
    let operator = &monkey.operator.unwrap().clone();
    let left_operand = get_number(monkeys, left_operand);
    let right_operand = get_number(monkeys, right_operand);
    match operator.as_str() {
        "*" => left_operand * right_operand,
        "+" => left_operand + right_operand,
        "-" => left_operand - right_operand,
        "/" => left_operand / right_operand,
        _ => panic!("Unknown operator"),
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Number(isize),
    Unknown,
    Operation(String, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn contains_unknown(&self) -> bool {
        match self {
            Expression::Number(_) => false,
            Expression::Unknown => true,
            Expression::Operation(_, left, right) => {
                left.contains_unknown() || right.contains_unknown()
            }
        }
    }
    fn get_lhs(&self) -> Expression {
        match self {
            Expression::Number(_) => panic!("Number on left hand side"),
            Expression::Unknown => panic!("Unknown on left hand side"),
            Expression::Operation(_, left, _) => *left.clone(),
        }
    }
    fn get_rhs(&self) -> Expression {
        match self {
            Expression::Number(_) => panic!("Number on right hand side"),
            Expression::Unknown => panic!("Unknown on right hand side"),
            Expression::Operation(_, _, right) => *right.clone(),
        }
    }
}

fn to_expression(monkeys: &HashMap<String, Monkey>, name: &str) -> Expression {
    let monkey = monkeys.get(name).unwrap().clone();
    let value = monkey.value;
    if name == "humn" {
        return Expression::Unknown;
    }
    if value.is_some() {
        return Expression::Number(value.unwrap());
    }
    let left_operand = &monkey.left_operand.unwrap().clone();
    let right_operand = &monkey.right_operand.unwrap().clone();
    let operator = &monkey.operator.unwrap().clone();

    let left_operand = to_expression(monkeys, left_operand);
    let right_operand = to_expression(monkeys, right_operand);

    if name == "root" {
        return Expression::Operation(
            "=".to_string(),
            Box::new(left_operand),
            Box::new(right_operand),
        );
    }

    Expression::Operation(
        operator.to_string(),
        Box::new(left_operand),
        Box::new(right_operand),
    )
}

fn solve_equation(lhs: Expression, rhs: Expression) -> isize {
    // left hand side must be the side with the unknown
    if rhs.contains_unknown() {
        return solve_equation(rhs, lhs);
    }

    let rhs_value = evaluate_expression(rhs);
    match lhs {
        Expression::Operation(operator, left, right) => match operator.as_str() {
            "+" => {
                if left.contains_unknown() {
                    solve_equation(
                        *left,
                        Expression::Number(rhs_value - evaluate_expression(*right)),
                    )
                } else {
                    solve_equation(
                        *right,
                        Expression::Number(rhs_value - evaluate_expression(*left)),
                    )
                }
            }
            "-" => {
                if left.contains_unknown() {
                    solve_equation(
                        *left,
                        Expression::Number(rhs_value + evaluate_expression(*right)),
                    )
                } else {
                    solve_equation(
                        *right,
                        Expression::Number(evaluate_expression(*left) - rhs_value),
                    )
                }
            }
            "*" => {
                if left.contains_unknown() {
                    solve_equation(
                        *left,
                        Expression::Number(rhs_value / evaluate_expression(*right)),
                    )
                } else {
                    solve_equation(
                        *right,
                        Expression::Number(rhs_value / evaluate_expression(*left)),
                    )
                }
            }
            "/" => {
                if left.contains_unknown() {
                    solve_equation(
                        *left,
                        Expression::Number(rhs_value * evaluate_expression(*right)),
                    )
                } else {
                    solve_equation(
                        *right,
                        Expression::Number(evaluate_expression(*left) / rhs_value),
                    )
                }
            }
            _ => panic!("Unknown operator"),
        },
        Expression::Unknown => rhs_value,
        Expression::Number(_) => panic!("Number on left hand side"),
    }
}

fn evaluate_expression(expression: Expression) -> isize {
    match expression {
        Expression::Number(n) => n,
        Expression::Unknown => panic!("Unknown value"),
        Expression::Operation(o, l, r) => match o.as_str() {
            "+" => evaluate_expression(*l) + evaluate_expression(*r),
            "-" => evaluate_expression(*l) - evaluate_expression(*r),
            "*" => evaluate_expression(*l) * evaluate_expression(*r),
            "/" => evaluate_expression(*l) / evaluate_expression(*r),
            _ => panic!("Unknown operator"),
        },
    }
}
