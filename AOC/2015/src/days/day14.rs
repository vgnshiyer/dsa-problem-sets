fn calculate_travel_distance(speed: u32, travel_time: u32, rest_time: u32, total_time: u32) -> u32 {
    speed * (total_time / (travel_time + rest_time)) * travel_time + speed * std::cmp::min(travel_time, total_time % (travel_time + rest_time))
}

pub fn part1(speed_list: &Vec<String>) -> u32 {
    let mut max_distance: u32 = 0;
    for speed in speed_list {
        let parts: Vec<&str> = speed.split_whitespace().collect();
        let speed: u32 = parts[3].parse().unwrap();
        let travel_time: u32 = parts[6].parse().unwrap();
        let rest_time: u32 = parts[13].parse().unwrap();
        max_distance = max_distance.max(calculate_travel_distance(speed, travel_time, rest_time, 2503));
    }
    max_distance
}

#[allow(dead_code)]
#[derive(Debug)]
struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    travel_time: u32,
    rest_time: u32,
    timer: u32,
    points: u32,
    distance_travelled: u32,
    is_resting: bool
}

impl<'a> Reindeer<'a> {
    fn new(name: &'a str, speed: u32, travel_time: u32, rest_time: u32) -> Reindeer<'a> {
        Reindeer {
            name,
            speed,
            travel_time,
            rest_time,
            timer: 1,
            points: 0,
            distance_travelled: 0,
            is_resting: false
        }
    }
}

fn simulate_race(time: u32, reindeers: &mut Vec<Reindeer>) {
    for _ in 0..time {
        let mut leading_dist: u32 = 0;
        for reindeer in &mut *reindeers {
            reindeer.timer += 1;
            reindeer.distance_travelled += if reindeer.is_resting { 0 } else { reindeer.speed };

            if reindeer.is_resting {
                if reindeer.timer > reindeer.rest_time {
                    reindeer.is_resting = false;
                    reindeer.timer = 1;
                }
            } else {
                if reindeer.timer > reindeer.travel_time {
                    reindeer.is_resting = true;
                    reindeer.timer = 1;
                }
            }

            leading_dist = leading_dist.max(reindeer.distance_travelled);
        }

        for reindeer in &mut *reindeers {
            if reindeer.distance_travelled == leading_dist {
                reindeer.points += 1;
            }
        }
    }
}

pub fn part2(speed_list: &Vec<String>) -> u32 {
    let mut reindeers: Vec<Reindeer> = Vec::new();
    let mut max_points: u32 = 0;
    for speed in speed_list {
        let parts: Vec<&str> = speed.split_whitespace().collect();
        let name: &str = parts[0];
        let speed: u32 = parts[3].parse().unwrap();
        let travel_time: u32 = parts[6].parse().unwrap();
        let rest_time: u32 = parts[13].parse().unwrap();

        reindeers.push(Reindeer::new(name, speed, travel_time, rest_time));
    }

    simulate_race(2503, &mut reindeers);
    for reindeer in &reindeers {
        max_points = max_points.max(reindeer.points);
    }
    max_points
}