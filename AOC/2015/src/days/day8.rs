fn get_code_chars(string: &String) -> usize {
    string.len()
}

fn get_memory_chars(string: &String) -> usize {
    let mut memory_chars = string.len() - 2;
    let chars: Vec<char> = string.chars().collect();
    let mut i = 1;
    loop {
        if i > chars.len() - 2 {
            break;
        }
        if chars[i] == '\\' {
            if chars[i + 1] == '\\' || chars[i + 1] == '"' {
                memory_chars -= 1;
                i += 2;
            } else if chars[i + 1] == 'x' {
                memory_chars -= 3;
                i += 4;
            }
            continue;
        }
        i += 1;
    }
    memory_chars
}

fn get_to_encode(string: &String) -> usize {
    let mut to_encode = string.len() + 2;
    for c in string.chars() {
        if c == '\\' || c == '"' {
            to_encode += 1;
        }
    }
    to_encode
}

pub fn part1(strings: &Vec<String>) -> usize {
    let mut total_code_chars: usize = 0;
    let mut total_memory_chars: usize = 0;
    for string in strings {
        total_code_chars += get_code_chars(string);
        total_memory_chars += get_memory_chars(string);
    }
    total_code_chars - total_memory_chars
}

pub fn part2(strings: &Vec<String>) -> usize {
    let mut total_code_chars: usize = 0;
    let mut to_encode: usize = 0;
    for string in strings {
        total_code_chars += get_code_chars(string);
        to_encode += get_to_encode(string);
    }
    to_encode - total_code_chars
}