fn change_next_state(current_state: &Vec<Vec<char>>, next_state: &mut Vec<Vec<char>>, light: (isize, isize)) {
    let d: Vec<(isize, isize)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
    let mut on_count: u32 = 0;
    for (dx, dy) in d {
        let x: isize = light.0 + dx;
        let y: isize = light.1 + dy;

        if x >= 0 && x < current_state.len() as isize && y >= 0 && y < current_state[0].len() as isize {
            if current_state[x as usize][y as usize] == '#' {
                on_count += 1;
            }
        }
    }

    let is_on: bool = current_state[light.0 as usize][light.1 as usize] == '#';
    if is_on {
        if on_count == 2 || on_count == 3 {
            next_state[light.0 as usize][light.1 as usize] = '#';
        } else {
            next_state[light.0 as usize][light.1 as usize] = '.';
        }
    } else {
        if on_count == 3 {
            next_state[light.0 as usize][light.1 as usize] = '#';
        }
    }
}

fn how_many_on(current_state: &Vec<Vec<char>>) -> u32 {
    let n = current_state.len();
    let m = current_state[0].len();
    let mut on: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            on += (current_state[i][j] == '#') as u32;
        }
    }
    on
}

pub fn part1(light_grid: &Vec<String>) -> u32 {
    let mut current_state: Vec<Vec<char>> = light_grid.iter().map(|s| s.chars().collect()).collect();
    let n = current_state.len();
    let m = current_state[0].len();
    for _ in 0..100 {
        let mut next_state = current_state.clone();
        for i in 0..n as isize {
            for j in 0..m as isize {
                change_next_state(&current_state, &mut next_state, (i, j));
            }
        }
        current_state = next_state;
    }
    how_many_on(&current_state)
}

pub fn part2(light_grid: &Vec<String>) -> u32 {
    let mut current_state: Vec<Vec<char>> = light_grid.iter().map(|s| s.chars().collect()).collect();
    let n = current_state.len() as isize;
    let m = current_state[0].len() as isize;
    // corners always on
    current_state[0][0] = '#';
    current_state[0][(m - 1) as usize] = '#';
    current_state[(n - 1) as usize][(m - 1) as usize] = '#';
    current_state[(n - 1) as usize][0] = '#';

    for _ in 0..100 {
        let mut next_state = current_state.clone();
        for i in 0..n {
            for j in 0..m {
                let is_corner: bool = (i, j) == (0, 0) || (i, j) == (0, m - 1) || (i, j) == (n - 1, m - 1) || (i, j) == (n - 1, 0);
                if is_corner {
                    continue;
                }
                change_next_state(&current_state, &mut next_state, (i, j));
            }
        }
        current_state = next_state;
    }
    how_many_on(&current_state)  
}