use std::fmt::format;

fn compute_hash(value: &str) -> String {
    let digest = md5::compute(value);
    format!("{:x}", digest)
}

fn starts_with_5_zeros(hash: &str) -> bool {
    hash.chars().take(5).all(|c| c == '0')
}

fn starts_with_6_zeros(hash: &str) -> bool {
    hash.chars().take(6).all(|c| c == '0')
}

pub fn part1(secret_key: &str) -> u64 {
    let mut i: u64 = 0;
    loop {
        let input = format!("{}{}", secret_key, i);
        let hash = compute_hash(&input);
        if starts_with_5_zeros(&hash) {
            return i;
        }
        i += 1;
    }
    0
}

pub fn part2(secret_key: &str) -> u64 {
    let mut i: u64 = 0;
    loop {
        let input = format!("{}{}", secret_key, i);
        let hash = compute_hash(&input);
        if starts_with_6_zeros(&hash) {
            return i;
        }
        i += 1;
    }
    0
}