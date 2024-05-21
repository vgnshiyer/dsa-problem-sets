fn look_and_say(starting_seq: &str, times: u8) -> u32 {
    let mut seq: String = starting_seq.to_string();
    for _ in 0..times {
        let mut f: u32 = 1;
        let chars: Vec<char> = seq.chars().collect();
        let mut temp = String::new();
        for i in 1..chars.len() {
            if chars[i] != chars[i - 1] {
                temp += &format!("{f}{}", chars[i - 1]);
                f = 1;
            } else {
                f += 1;
            }
        }
        temp += &format!("{f}{}",chars[chars.len() - 1]);
        seq = temp.clone();
    }
    seq.len().try_into().unwrap()
}

pub fn part1(starting_seq: &str) -> u32 {
    look_and_say(starting_seq, 40)
}

pub fn part2(starting_seq: &str) -> u32 {
    look_and_say(starting_seq, 50)
}