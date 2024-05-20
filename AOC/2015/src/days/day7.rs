use std::{collections::HashMap, fmt::format};
use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
struct Equation {
    op1: String,
    op2: String,
    op: String
}

impl Equation {
    fn new(op1: String, op2: String, op: String) -> Self {
        if !["NOT", "RSHIFT", "LSHIFT", "AND", "OR", "XOR", ""].contains(&op.as_str()) {
            panic!("Unsupported operand: {op1}{op}{op2}");
        }

        Equation {
            op1: op1,
            op2: op2,
            op: op
        }
    }
}

fn parse(equation: &str, unsolved: &mut HashMap<String, Vec<Equation>>, solved: &mut HashMap<String, i32>) {
    let parts: Vec<&str> = equation.split_whitespace().collect();
    match parts.len() {
        3 => {
            // 126 -> x / y -> x
            match parts[0].parse::<i32>() {
                Ok(val) => {
                    // parts[0] is a number
                    solved.insert(parts[2].to_string(), val);
                },
                Err(_) => {
                    // parts[0] is a variable
                    unsolved.entry(parts[2].to_string()).or_insert_with(Vec::new)
                    .push(Equation::new(
                        parts[0].to_string(), 
                        "".to_string(), 
                        "".to_string()));
                },
            }
        },
        4 => {
            // NOT
            unsolved.entry(parts[3].to_string()).or_insert_with(Vec::new)
            .push(Equation::new(
                parts[1].to_string(),
                "".to_string(),
                parts[0].to_string()));
        },
        5 => {
            // AND, OR, XOR, RSHIFT, LSHIFT
            unsolved.entry(parts[4].to_string()).or_insert_with(Vec::new)
            .push(Equation::new(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[1].to_string()));
        },
        _ => {}
    };
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn solve(for_: &String, solved: &mut HashMap<String, i32>, unsolved: &mut HashMap<String, Vec<Equation>>) {
    if solved.contains_key(for_) {
        return;
    }

    if let Some(equations) = unsolved.get(for_).cloned() {
        for eq in equations {
            if !solved.contains_key(&eq.op1) && !is_number(&eq.op1) {
                solve(&eq.op1, solved, unsolved);
            }
            if !eq.op2.is_empty() && !solved.contains_key(&eq.op2)  && !is_number(&eq.op2) {
                solve(&eq.op2, solved, unsolved);
            }
            let result = compute(&eq, solved);
            solved.insert(for_.clone(), result);
        }
    }
}

fn compute(equation: &Equation, solved: &HashMap<String, i32>) -> i32 {
    let op: &str = &equation.op;
    let val1 = match equation.op1.parse::<i32>() {
        Ok(val) => val,
        Err(_) => solved.get(&equation.op1).unwrap().clone()
    };
    let val2 = match equation.op2.parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            if equation.op2.is_empty() {
                0
            } else {
                solved.get(&equation.op2).unwrap().clone()
            }
        }
    };
    match op {
        "NOT" => !val1,
        "AND" => val1 & val2,
        "OR" => val1 | val2,
        "XOR" => val1 ^ val2,
        "LSHIFT" => val1 << val2,
        "RSHIFT" => val1 >> val2,
        "" => val1,
        _ => panic!("Unknown error: {:#?}", equation),
    }
}

pub fn part1(connections: &Vec<String>) -> i32 {
    let mut unsolved: HashMap<String, Vec<Equation>> = HashMap::new();
    let mut solved: HashMap<String, i32> = HashMap::new();

    for con in connections {
        parse(con, &mut unsolved, &mut solved);
    }
    
    let for_ = String::from("a");
    solve(&for_, &mut solved, &mut unsolved);
    *solved.get(&for_).unwrap()
}

pub fn part2(connections: &Vec<String>, override_b: i32) -> i32 {
    let mut unsolved: HashMap<String, Vec<Equation>> = HashMap::new();
    let mut solved: HashMap<String, i32> = HashMap::new();

    for con in connections {
        parse(con, &mut unsolved, &mut solved);
    }
    solved.insert("b".to_string(), override_b);

    let for_ = String::from("a");
    solve(&for_, &mut solved, &mut unsolved);
    *solved.get(&for_).unwrap()
}