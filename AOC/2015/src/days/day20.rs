pub fn part1(num_presents: usize) -> usize {
    let mut houses: Vec<usize> = vec![0; 1000000];
    for elf in 1..1000000 {
        let mut house_num: usize = elf;
        while house_num <= houses.len() - 1 {
            houses[house_num] += elf * 10;
            house_num += elf;
        }
    }

    for i in 0..houses.len() {
        if houses[i] >= num_presents {
            return i;
        }
    }
    0
}

pub fn part2(num_presents: usize) -> usize {
    let mut houses: Vec<usize> = vec![0; 1000000];
    for elf in 1..1000000 {
        let mut house_num: usize = elf;
        let mut presents_delivered: u32 = 0;
        while presents_delivered < 50 && house_num < houses.len() {
            houses[house_num] += elf * 11;
            house_num += elf;
            presents_delivered += 1;
        }
    }

    for i in 0..houses.len() {
        if houses[i] >= num_presents {
            return i;
        }
    }
    0
}