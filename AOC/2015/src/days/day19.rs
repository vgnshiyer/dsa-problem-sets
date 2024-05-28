use std::{collections::{HashMap, HashSet}, f32::INFINITY};

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

fn dfs(min_steps: &mut u32, num_steps: u32, molecule: String, medicine_molecule: String, replacements: &HashMap<String, Vec<String>>) {
    if molecule.len() > medicine_molecule.len() {
        return;
    }
    if molecule == medicine_molecule {
        *min_steps = std::cmp::min(*min_steps, num_steps);
        return;
    }

    for (key, values) in replacements {
        if key.len() <= molecule.len() {
            for i in 0..molecule.len() - key.len() + 1 {
                if &molecule[i..i+key.len()] == key {
                    for replacement in values {
                        let mut new_molecule: String = molecule.clone();
                        new_molecule.replace_range(i..i+key.len(), replacement);
                        dfs(min_steps, num_steps + 1, new_molecule, medicine_molecule.clone(), replacements)
                    }
                }
            }
        }
    }
}