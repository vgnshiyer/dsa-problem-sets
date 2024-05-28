use std::collections::{HashMap, HashSet};

pub fn part1(inp: &Vec<String>) -> u32 {
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
    let starting_molecule: String = inp[inp.len() - 1].clone();
    let mut distinct_molecules: HashSet<String> = HashSet::new();

    for i in 0..inp.len() - 2 {
        let parts: Vec<String> = inp[i].split(" => ").map(|s| s.to_string()).collect();
        replacements.entry(parts[0].to_string()).or_insert_with(Vec::new)
                    .push(parts[1].to_string());
    }

    for (key, values) in replacements {
        for i in 0..starting_molecule.len() - key.len() + 1 {
            if &starting_molecule[i..i+key.len()] == key {
                for replacement in &values {
                    let mut new_molecule: String = starting_molecule.clone();
                    new_molecule.replace_range(i..i+key.len(), replacement);
                    distinct_molecules.insert(new_molecule);
                }
            }
        }
    }

    distinct_molecules.len() as u32
}