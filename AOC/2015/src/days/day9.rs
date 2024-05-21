use std::{collections::HashMap, f32::{INFINITY, NEG_INFINITY}};

fn tsp(prev: usize, mask: usize, n: usize, dp: &mut HashMap<(usize, usize), f32>, adj: &Vec<Vec<f32>>) -> f32 {
    if dp.contains_key(&(prev, mask)) {
        return dp.get(&(prev, mask)).unwrap().clone();
    }
    if mask == (1 << n) - 1 {
        return 0.0;
    }

    let mut cost = INFINITY;
    for i in 0..n as usize {
        if (mask & (1 << i)) != 0 {
            continue;
        }
        cost = cost.min(adj[prev][i] + tsp(i, mask | (1 << i), n, dp, adj));
    }
    dp.insert((prev, mask), cost);
    cost
}

fn get_next_index(idx: &mut usize, city_index: &mut HashMap<String, usize>, city_name: String) -> usize {
    if !city_index.contains_key(&city_name) {
        city_index.insert(city_name.clone(), *idx);
        *idx += 1;
    }
    *city_index.get(&city_name).unwrap() as usize
}

pub fn part1(distances: &Vec<String>) -> f32 {
    let mut idx: usize = 0;
    let mut adj: Vec<Vec<f32>> = vec![vec![INFINITY; 10]; 10];
    let mut city_index: HashMap<String, usize> = HashMap::new();

    for dist in distances {
        let parts: Vec<&str> = dist.split_whitespace().collect();
        let from: String = parts[0].to_string();
        let from_idx = get_next_index(&mut idx, &mut city_index, from);
        let to: String = parts[2].to_string();
        let to_idx = get_next_index(&mut idx, &mut city_index, to);
        let d: f32 = parts[parts.len() - 1].parse().unwrap();

        adj[from_idx][to_idx] = d;
        adj[to_idx][from_idx] = d;

        adj[from_idx][from_idx] = 0.0;
        adj[to_idx][to_idx] = 0.0;
    }

    let n = city_index.len();
    let mut cost: f32 = INFINITY;
    let mut dp: HashMap<(usize, usize), f32> = HashMap::new();
    for i in 0..n {
        cost = cost.min(tsp(i, 1 << i, n, &mut dp, &adj));
    }
    cost
}

fn tsp_inverse(prev: usize, mask: usize, n: usize, dp: &mut HashMap<(usize, usize), f32>, adj: &Vec<Vec<f32>>) -> f32 {
    if dp.contains_key(&(prev, mask)) {
        return dp.get(&(prev, mask)).unwrap().clone();
    }
    if mask == (1 << n) - 1 {
        return 0.0;
    }

    let mut cost = NEG_INFINITY;
    for i in 0..n as usize {
        if (mask & (1 << i)) != 0 {
            continue;
        }
        cost = cost.max(adj[prev][i] + tsp_inverse(i, mask | (1 << i), n, dp, adj));
    }
    dp.insert((prev, mask), cost);
    cost
}

pub fn part2(distances: &Vec<String>) -> f32 {
    let mut idx: usize = 0;
    let mut adj: Vec<Vec<f32>> = vec![vec![NEG_INFINITY; 10]; 10];
    let mut city_index: HashMap<String, usize> = HashMap::new();

    for dist in distances {
        let parts: Vec<&str> = dist.split_whitespace().collect();
        let from: String = parts[0].to_string();
        let from_idx = get_next_index(&mut idx, &mut city_index, from);
        let to: String = parts[2].to_string();
        let to_idx = get_next_index(&mut idx, &mut city_index, to);
        let d: f32 = parts[parts.len() - 1].parse().unwrap();

        adj[from_idx][to_idx] = d;
        adj[to_idx][from_idx] = d;

        adj[from_idx][from_idx] = 0.0;
        adj[to_idx][to_idx] = 0.0;
    }

    let n = city_index.len();
    let mut cost: f32 = NEG_INFINITY;
    let mut dp: HashMap<(usize, usize), f32> = HashMap::new();
    for i in 0..n {
        cost = cost.max(tsp_inverse(i, 1 << i, n, &mut dp, &adj));
    }
    cost
}