fn has_increasing_seq_of_3_chars(password: &str) -> bool {
    let mut i: usize = 0;
    let chars: Vec<char> = password.chars().collect();
    loop {
        if i >= chars.len() - 2 {
            return false;
        }
        if chars[i] < chars[i + 1] && chars[i + 1] < chars[i + 2] {
            if (chars[i + 1] as u8) - (chars[i] as u8) == 1 && (chars[i + 2] as u8) - (chars[i + 1] as u8) == 1 {
                return true;
            }
        }
        i += 1;
    }
}

fn not_contain_iol(password: &str) -> bool {
    !(password.contains("i") || password.contains("o") || password.contains("l"))
}

fn contains_2pairs(password: &str) -> bool {
    let mut i: usize = 0;
    let chars: Vec<char> = password.chars().collect();
    let mut pairs: u8 = 0;
    loop {
        if i >= chars.len() - 1 {
            return pairs >= 2;
        }
        if chars[i] == chars[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
}

fn update_password(password: &mut String) {
    let mut chars: Vec<char> = password.chars().collect();
    let mut i: usize = chars.len() - 1;
    loop {
        if chars[i] == 'z' {
            chars[i] = 'a';
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            chars[i] = ((chars[i] as u8) + 1) as char;
            break;
        }
    }
    *password = chars.into_iter().collect();
}

pub fn part1(current_password: &str) -> String {
    let mut next_password = current_password.to_string();
    loop {
        update_password(&mut next_password);
        if has_increasing_seq_of_3_chars(&next_password) && not_contain_iol(&next_password) && contains_2pairs(&next_password) {
            return next_password;
        }
    }
}