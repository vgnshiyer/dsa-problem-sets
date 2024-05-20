use std::{
    env, fs::{self, read}, io
};

mod days;

fn read_input(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Error while reading file.");
    content.split('\n').map(|s| s.to_string()).collect()
}

fn get_day(day_num: u32) -> String {
    if day_num >=1 && day_num <= 25 {
        format!("day{}", day_num)
    } else {
        panic!("Invalid day number. Must be between 1 and 25.")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut day_num: String = String::new();
    if args.len() >= 2 {
        day_num.clone_from(&args[1]);
    } else {
        println!("Enter the day number: ");
        io::stdin()
            .read_line(&mut day_num)
            .expect("Enter a valid number");
    }

    let day_num: u32 = match day_num.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let cwd = env::current_dir().unwrap();
    let temp = cwd.join(format!("input/{}.txt", get_day(day_num)));
    let filename = temp.to_str().expect("Invalid UTF-8 sequence in file path");

    match day_num {
        1 => {
            let input_sequence: String = read_input(&filename).get(0).expect("No input found").clone();
            println!("Part1: Floor number: {}", days::day1::part1(&input_sequence));
            println!("Part2: Position at floor -1: {}", days::day1::part2(&input_sequence));
        },
        2 => {
            let box_dimensions = read_input(&filename);
            println!("Part 1: Wrapping paper needed: {}", days::day2::part1(&box_dimensions));
            println!("Part 2: Ribbon needed: {}", days::day2::part2(&box_dimensions));
        },
        3 => {
            let directions: String = read_input(&filename).get(0).expect("Invalid input directions").clone();
            println!("Part 1: Presents delivered: {}", days::day3::part1(&directions));
            println!("Part 2: Presents delivered: {}", days::day3::part2(&directions));
        },
        4 => {
            let secret_key: String = read_input(&filename).get(0).expect("Invalid input").clone();
            println!("Part1: Smallest number: {}", days::day4::part1(&secret_key));
            println!("Part2: Smallest number: {}", days::day4::part2(&secret_key));
        },
        5 => {
            let strings = read_input(&filename);
            println!("Part 1: Nice strings: {}", days::day5::part1(&strings));
            println!("Part 2: Nice strings: {}", days::day5::part2(&strings));
        },
        6 => {
            let instructions = read_input(&filename);
            println!("Part1: Lights lit: {}", days::day6::part1(&instructions));
            println!("Part2: Lights lit: {}", days::day6::part2(&instructions));
        },
        7 => {
            let connections: Vec<String> = read_input(&filename);
            let part1_res = days::day7::part1(&connections);
            println!("Part1: Signal in wire a: {}", part1_res);
            println!("Part2: New Signal in wire a: {}", days::day7::part2(&connections, part1_res));
        },
        8 => {
            let strings: Vec<String> = read_input(&filename);
            println!("Part1: {}", days::day8::part1(&strings));
            println!("Part2: {}", days::day8::part2(&strings));
        },
        _ => println!("The Day number is either invalid or is not implemented.")
    }
}
