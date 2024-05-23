use std::{cmp, collections::HashMap, f32::NEG_INFINITY};

fn get_next_index(idx: &mut usize, guest_index: &mut HashMap<String, usize>, guest_name: &str) -> usize {
    if !guest_index.contains_key(guest_name) {
        guest_index.insert(guest_name.to_string(), *idx);
        *idx += 1;
    }
    *guest_index.get(guest_name).unwrap() as usize
}

fn calculate_happiness(arrangement: &mut Vec<usize>, adj: &Vec<Vec<i32>>) -> i32 {
    let mut total_happiness = 0;
    for i in 0..arrangement.len() {
        let guest1 = arrangement[i];
        let guest2 = arrangement[(i + 1) % arrangement.len()];
        total_happiness += adj[guest1][guest2];
    }
    total_happiness
}

fn permute(arrangement: &mut Vec<usize>, remaining: &mut Vec<usize>, adj: &Vec<Vec<i32>>, max_happiness: &mut i32) {
    if remaining.is_empty() {
        *max_happiness = cmp::max(*max_happiness, calculate_happiness(arrangement, adj));
    } else {
        for i in 0..remaining.len() {
            let guest = remaining.remove(i);
            arrangement.push(guest);
            permute(arrangement, remaining, adj, max_happiness);
            arrangement.pop();
            remaining.insert(i, guest);
        }
    }
}

pub fn part1(guest_list: &Vec<String>) -> i32 {
    let mut guest_index: HashMap<String, usize> = HashMap::new();
    let mut idx: usize = 0;
    let mut adj: Vec<Vec<i32>> = vec![vec![0; 10]; 10];

    for guest_happiness in guest_list {
        let parts: Vec<&str> = guest_happiness.split_whitespace().collect();
        let guest1: usize = get_next_index(&mut idx, &mut guest_index, parts[0]);
        let guest2: usize = get_next_index(&mut idx, &mut guest_index, parts[parts.len() - 1].trim_end_matches("."));
        let happiness: i32 = parts[3].parse().unwrap();
        let is_gain: bool = parts[2] == "gain";

        adj[guest1][guest2] += if is_gain { happiness } else { -happiness };
        adj[guest2][guest1] += if is_gain { happiness } else { -happiness };
    }
    
    let mut arrangement: Vec<usize> = Vec::new();
    let mut guests: Vec<usize> = (0..guest_index.len()).collect();
    let mut max_happiness: i32 = NEG_INFINITY as i32;
    permute(&mut arrangement, &mut guests, &adj, &mut max_happiness);
    max_happiness
}

pub fn part2(guest_list: &Vec<String>) -> i32 {
    let mut guest_index: HashMap<String, usize> = HashMap::new();
    let mut idx: usize = 0;
    let mut adj: Vec<Vec<i32>> = vec![vec![0; 10]; 10];
    
    for guest_happiness in guest_list {
        let parts: Vec<&str> = guest_happiness.split_whitespace().collect();
        let guest1: usize = get_next_index(&mut idx, &mut guest_index, parts[0]);
        let guest2: usize = get_next_index(&mut idx, &mut guest_index, parts[parts.len() - 1].trim_end_matches("."));
        let happiness: i32 = parts[3].parse().unwrap();
        let is_gain: bool = parts[2] == "gain";
        
        adj[guest1][guest2] += if is_gain { happiness } else { -happiness };
        adj[guest2][guest1] += if is_gain { happiness } else { -happiness };
    }
    
    let mut arrangement: Vec<usize> = Vec::new();

    // adding myself to the list
    get_next_index(&mut idx, &mut guest_index, "Myself");
    let mut guests: Vec<usize> = (0..guest_index.len()).collect();
    let mut max_happiness: i32 = NEG_INFINITY as i32;
    permute(&mut arrangement, &mut guests, &adj, &mut max_happiness);
    max_happiness
}