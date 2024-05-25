use std::{collections::HashMap, f32::INFINITY};

#[derive(Debug)]
struct Traits {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Traits {
    fn get(&self, key: &str) -> Option<u32> {
        match key {
            "children" => self.children,
            "cats" => self.cats,
            "samoyeds" => self.samoyeds,
            "pomeranians" => self.pomeranians,
            "akitas" => self.akitas,
            "vizslas" => self.vizslas,
            "goldfish" => self.goldfish,
            "trees" => self.trees,
            "cars" => self.cars,
            "perfumes" => self.perfumes,
            _ => None
        }
    }
}

fn parse_sue_data(sue_traits: &String, sues: &mut Vec<Traits>) {
    let parts: Vec<&str> = sue_traits.split_whitespace().collect();
    let traits_str: String = parts[2..].join(" ");
    let traits_map: HashMap<String, u32> = traits_str.split(", ").map(|t| {
        let pair: Vec<&str> = t.split(": ").collect();
        (pair[0].to_string(), pair[1].parse().unwrap())
    }).collect();
    
    sues.push(Traits{
        children: traits_map.get("children").cloned(),
        cats: traits_map.get("cats").cloned(),
        samoyeds: traits_map.get("samoyeds").cloned(),
        pomeranians: traits_map.get("pomeranians").cloned(),
        akitas: traits_map.get("akitas").cloned(),
        vizslas: traits_map.get("vizslas").cloned(),
        goldfish: traits_map.get("goldfish").cloned(),
        trees: traits_map.get("trees").cloned(),
        cars: traits_map.get("cars").cloned(),
        perfumes: traits_map.get("perfumes").cloned(),
    });
}

const MFCSAM_SCAN: Traits = Traits {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

pub fn part1(sue_list: &Vec<String>) -> u32 {
    for (i, sue_traits) in sue_list.into_iter().enumerate() {
        let parts: Vec<&str> = sue_traits.split_whitespace().collect();
        let traits_str: String = parts[2..].join(" ");
        let traits: Vec<(String, u32)> = traits_str.split(", ").map(|t| {
            let pair: Vec<&str> = t.split(": ").collect();
            (pair[0].to_string(), pair[1].parse().unwrap())
        }).collect();
        
        let trait1_name = &traits[0].0;
        let trait1_val = traits[0].1;
        let trait2_name = &traits[1].0;
        let trait2_val = traits[1].1;
        let trait3_name = &traits[2].0;
        let trait3_val = traits[2].1;

        let check1: bool = trait1_val == MFCSAM_SCAN.get(trait1_name).unwrap();
        let check2: bool = trait2_val == MFCSAM_SCAN.get(trait2_name).unwrap();
        let check3: bool = trait3_val == MFCSAM_SCAN.get(trait3_name).unwrap();

        if check1 && check2 && check3 {
            return (i + 1) as u32;
        }
    }
    0
}

pub fn part2(sue_list: &Vec<String>) -> u32 {
    for (i, sue_traits) in sue_list.into_iter().enumerate() {
        let parts: Vec<&str> = sue_traits.split_whitespace().collect();
        let traits_str: String = parts[2..].join(" ");
        let traits: Vec<(String, u32)> = traits_str.split(", ").map(|t| {
            let pair: Vec<&str> = t.split(": ").collect();
            (pair[0].to_string(), pair[1].parse().unwrap())
        }).collect();
        
        let trait1_name = &traits[0].0;
        let trait1_val = traits[0].1;
        let trait2_name = &traits[1].0;
        let trait2_val = traits[1].1;
        let trait3_name = &traits[2].0;
        let trait3_val = traits[2].1;

        fn check(trait_name: &str, trait_val: u32) -> bool {
            if trait_name == "cats" || trait_name == "trees" {
                trait_val > MFCSAM_SCAN.get(trait_name).unwrap()
            } else if trait_name == "pomeranians" || trait_name == "goldfish" {
                trait_val < MFCSAM_SCAN.get(trait_name).unwrap()
            } else {
                trait_val == MFCSAM_SCAN.get(trait_name).unwrap()
            }
        }

        let check1: bool = check(trait1_name, trait1_val);
        let check2: bool = check(trait2_name, trait2_val);
        let check3: bool = check(trait3_name, trait3_val);

        if check1 && check2 && check3 {
            return (i + 1) as u32;
        }
    }
    0
}