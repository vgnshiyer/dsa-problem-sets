use std::collections::HashSet;

pub fn part1(directions: &str) -> u32 {
    let mut presents: u32 = 1;
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    set.insert((x, y));
    for d in directions.chars() {
        let (dx, dy): (i32, i32) = match d {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => (0, 0)
        };
        x += dx;
        y += dy;

        if !set.contains(&(x, y)) {
            presents += 1;
        }
        set.insert((x, y));
    }
    presents
}

pub fn part2(directions: &str) -> u32 {
    let mut presents: u32 = 1;
    let mut santa: (i32, i32) = (0, 0);
    let mut robo_santa: (i32, i32) = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(santa);
    
    for (i, d) in directions.chars().enumerate() {
        let (dx, dy): (i32, i32) = match d {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => (0, 0)
        };
        if i % 2 == 0 {
            santa.0 += dx;
            santa.1 += dy;

            if !set.contains(&santa) {
                presents += 1;
            }
        } else {
            robo_santa.0 += dx;
            robo_santa.1 += dy;

            if !set.contains(&robo_santa) {
                presents += 1;
            }
        }

        set.insert(santa);
        set.insert(robo_santa);
    }
    presents
}