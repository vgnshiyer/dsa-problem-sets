fn find_paper_len_needed((l, h, w): (u32, u32, u32)) -> u32 {
    let surface_area = 2 * l * h + 2 * l * w + 2 * h * w;
    let mut side_areas = vec![l * h, l * w, h * w];
    side_areas.sort();
    let extra_paper = side_areas[0];
    surface_area + extra_paper
}

fn find_ribbon_len_needed((l, h, w): (u32, u32, u32)) -> u32 {
    let volume = l * h * w;
    let mut sides = vec![l, h, w];
    sides.sort();
    volume + 2 * (sides[0]) + 2 * sides[1]
}

fn get_dimensions(bd: &String) -> (u32, u32, u32) {
    let dimensions: Vec<&str> = bd.split("x").collect();
    let l: u32 = dimensions[0].trim().parse().expect("Invalid dimension");
    let h: u32 = dimensions[1].trim().parse().expect("Invalid dimension");
    let w: u32 = dimensions[2].trim().parse().expect("Invalid dimension");
    (l, h, w)
}

pub fn part1(box_dimensions: &Vec<String>) -> u32 {
    let mut wrapping_paper_len: u32 = 0;
    for bd in box_dimensions {
        wrapping_paper_len += find_paper_len_needed(get_dimensions(bd));
    }
    wrapping_paper_len
}

pub fn part2(box_dimensions: &Vec<String>) -> u32 {
    let mut ribbon_len = 0;
    for bd in box_dimensions {
        ribbon_len += find_ribbon_len_needed(get_dimensions(bd));
    }
    ribbon_len
}