use itertools::Itertools;

pub fn part1(container_capacities: &Vec<String>) -> u32 {
    let mut containers: Vec<usize> = Vec::new();
    for capacity in container_capacities {
        containers.push(capacity.parse().unwrap());
    }

    let mut ways: u32 = 0;
    for i in 1..=containers.len() {
        for c in containers.clone().into_iter().combinations(i) {
            if c.iter().map(|x| x).sum::<usize>() == 150 {
                ways += 1;
            }
        }
    }
    ways
}

pub fn part2(container_capacities: &Vec<String>) -> u32 {
    let mut containers: Vec<usize> = Vec::new();
    for capacity in container_capacities {
        containers.push(capacity.parse().unwrap());
    }

    let mut min_ways: u32 = 0;
    for i in 1..=containers.len() {
        for c in containers.clone().into_iter().combinations(i) {
            if c.iter().map(|x| x).sum::<usize>() == 150 {
                min_ways += 1;
            }
        }
        if min_ways != 0 {
            return min_ways;
        }
    }
    min_ways
}