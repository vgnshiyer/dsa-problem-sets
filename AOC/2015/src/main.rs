use std::{
    env, fs,
    io::{self, stdin},
};

mod days;

fn read_input(filename: &str) -> String {
    let content = fs::read_to_string(filename).expect("Error while reading file.");
    content.trim_end().to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();


    let mut day_num: String = String::new();
    if args.len() >= 2 {
        // day_num = args[1].clone();
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
    let filename = cwd.join(format!("input/{}.txt", day_num));
    println!("Reading {}", filename.display());

    let filename_str = filename.to_str().expect("Invalid UTF-8 sequence in file path");

    let input = read_input(filename_str);
    println!("Floor number: {}", days::day01::part1(input));
}
