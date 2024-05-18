enum Command {
    On,
    Off,
    Toggle   
}

struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position {
            x: x,
            y: y
        }
    }
}

struct Coordinate {
    start: Position,
    end: Position
}

fn get_command(instruction: &str) -> Command {
    if instruction.contains("turn on") {
        return Command::On;
    } else if instruction.contains("turn off") {
        return Command::Off;
    } else if instruction.contains("toggle") {
        return Command::Toggle;
    } else {
        panic!("Invalid instruction");
    }
}

fn get_coordinates(instruction: &str) -> Coordinate {
    let parts: Vec<&str> = instruction.split_whitespace().collect();
    let start: Vec<usize> = parts[parts.len() - 3].split(',').map(|s| s.parse().unwrap()).collect();
    let end: Vec<usize> = parts[parts.len() - 1].split(',').map(|s| s.parse().unwrap()).collect();
    Coordinate {
        start: Position::new(start[0], start[1]),
        end: Position::new(end[0], end[1])
    }
}

fn how_many_are_lit(lights: Vec<Vec<u32>>) -> u32 {
    let mut lights_lit = 0;
    for i in 0..lights.len() {
        for j in 0..lights[0].len() {
            lights_lit += (lights[i][j] == 1) as u32;
        }
    }
    lights_lit
}

pub fn part1(instructions: &Vec<String>) -> u32 {
    let mut lights: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for instruction in instructions {
        let command: Command = get_command(instruction);
        let coordinates = get_coordinates(instruction);
        
        for x in coordinates.start.x..=coordinates.end.x {
            for y in coordinates.start.y..=coordinates.end.y {
                match command {
                    Command::On => lights[x][y] = 1,
                    Command::Off => lights[x][y] = 0,
                    Command::Toggle => lights[x][y] ^= 1,
                }
            }
        }
    }
    how_many_are_lit(lights)
}

fn total_brightness(lights: Vec<Vec<u32>>) -> u32 {
    let mut brightness = 0;
    for i in 0..lights.len() {
        for j in 0..lights[0].len() {
            brightness += lights[i][j];
        }
    }
    brightness
}

pub fn part2(instructions: &Vec<String>) -> u32 {
    let mut lights: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for instruction in instructions {
        let command: Command = get_command(instruction);
        let coordinates = get_coordinates(instruction);
        
        for x in coordinates.start.x..=coordinates.end.x {
            for y in coordinates.start.y..=coordinates.end.y {
                match command {
                    Command::On => lights[x][y] += 1,
                    Command::Off => {
                        if lights[x][y] > 0 {
                            lights[x][y] -= 1;
                        } else {
                            lights[x][y] = 0;
                        }
                    },
                    Command::Toggle => lights[x][y] += 2,
                }
            }
        }
    }
    total_brightness(lights)
}