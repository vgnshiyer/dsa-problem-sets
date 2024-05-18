use std::fmt::format;

fn has_3_vowels(string: &str) -> bool {
    let mut vowels: u8 = 0;
    for c in string.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowels += 1;
        }
    }
    vowels >= 3
}

fn has_double_letters(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            return true;
        }
    }
    false
}

fn contains_disallowed_strings(string: &str) -> bool {
    let disallowed = vec!["ab", "cd", "pq", "xy"];
    for d in disallowed {
        if string.contains(d) {
            return true;
        }
    }
    false
}

pub fn part1(strings: &Vec<String>) -> u32 {
    let mut nice_strings: u32 = 0;
    for string in strings {
        if has_3_vowels(string) && has_double_letters(string) && !contains_disallowed_strings(string) {
            nice_strings += 1;
        }
    }
    nice_strings
}

fn contains_pair_that_appears_twice(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    for i in 0..chars.len() - 2 {
        let pair = format!("{}{}", chars[i], chars[i + 1]);
        for j in i+2..chars.len()-1 {
            if pair == format!("{}{}", chars[j], chars[j + 1]) {
                return true;
            }
        }
    }
    false
}

fn contains_special_triplet(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] && chars[i] != chars[i + 1] {
            return true;
        }
    }
    false
}

pub fn part2(strings: &Vec<String>) -> u32 {
    let mut nice_strings: u32 = 0;
    for string in strings {
       if contains_pair_that_appears_twice(string) && contains_special_triplet(string) {
            nice_strings += 1;
       } 
    }
    nice_strings
}