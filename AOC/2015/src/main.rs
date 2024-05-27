use std::{
    env, fs, io
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
        9 => {
            let distances: Vec<String> = read_input(&filename);
            println!("Part1: Shortest distance: {}", days::day9::part1(&distances));
            println!("Part1: Longest distance: {}", days::day9::part2(&distances));
        },
        10 => {
            let starting_seq = read_input(&filename).get(0).unwrap().clone();
            println!("Part 1: Length of sequence: {}", days::day10::part1(&starting_seq));
            println!("Part 2: Length of sequence: {}", days::day10::part2(&starting_seq));
        },
        11 => {
            let current_password: String = read_input(&filename).get(0).unwrap().clone();
            let next_password = days::day11::part1(&current_password);
            println!("Part 1: Next password: {}", next_password);
            println!("Part 1: Next password: {}", days::day11::part1(&next_password));
        },
        12 => {
            let json: String = read_input(&filename).get(0).unwrap().clone();
            println!("Part 1: Sum of numbers: {}", days::day12::part1(&json));
            println!("Part 2: Sum of numbers: {}", days::day12::part2(&json));
        },
        13 => {
            let guest_list: Vec<String> = read_input(&filename);
            println!("Part 1: total change in happiness: {}", days::day13::part1(&guest_list));
            println!("Part 2: total change in happiness: {}", days::day13::part2(&guest_list));
        },
        14 => {
            let speed_list: Vec<String> = read_input(&filename);
            println!("Part 1: distance travelled by winner: {}", days::day14::part1(&speed_list));
            println!("Part 2: points scored by winner: {}", days::day14::part2(&speed_list));
        },
        15 => {
            let ingredients: Vec<String> = read_input(&filename);
            println!("Part 1: total score of best cookie: {}", days::day15::part1(&ingredients));
            println!("Part 2: total score of best 500 Calorie cookie: {}", days::day15::part2(&ingredients));
        },
        16 => {
            let sue_list: Vec<String> = read_input(&filename);
            println!("Part 1: Aunt Sue who gave the gift: {}", days::day16::part1(&sue_list));
            println!("Part 2: Aunt Sue who gave the gift: {}", days::day16::part2(&sue_list));
        },
        17 => {
            let containers: Vec<String> = read_input(&filename);
            println!("Part 1: How many combinations? {}", days::day17::part1(&containers));
            println!("Part 2: How many combinations for min containers? {}", days::day17::part2(&containers));
        },
        18 => {
            let light_grid: Vec<String> =read_input(&filename);
            println!("Part 1: How many lights are on? {}", days::day18::part1(&light_grid));
            println!("Part 2: How many lights are on? {}", days::day18::part2(&light_grid));
        },
        _ => println!("The Day number is either invalid or is not implemented.")
    }
}
