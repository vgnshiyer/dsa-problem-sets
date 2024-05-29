use std::f32::INFINITY;
use std::cmp::max;

fn boss_fight(player: (i32, i32, i32)) -> bool {
    let mut boss = (104, 8, 1);
    let mut player_hp = player.0;
    let player_damage = player.1;
    let player_armor = player.2;

    loop {
        boss.0 -= max(player_damage - boss.2, 1);
        if boss.0 <= 0 {
            return true;
        }
        player_hp -= max(boss.1 - player_armor, 1);
        if player_hp <= 0 {
            return false;
        }
    }
}

pub fn part1(_: &Vec<String>) -> i32 {
    let weapons = vec![
        (8, 4, 0),
        (10, 5, 0),
        (25, 6, 0),
        (40, 7, 0),
        (74, 8, 0),
    ];

    let armors = vec![
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];

    let rings = vec![
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let mut min_cost: i32 = INFINITY as i32;

    for weapon in &weapons {
        for armor in &armors {
            for i in 0..rings.len() {
                for j in (i + 1)..rings.len() {
                    let ring1 = rings[i];
                    let ring2 = rings[j];
                    let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                    let damage = weapon.1 + ring1.1 + ring2.1;
                    let armor_value = armor.2 + ring1.2 + ring2.2;

                    if boss_fight((100, damage, armor_value)) {
                        min_cost = min_cost.min(cost);
                    }
                }
            }
        }
    }
    
    min_cost
}

pub fn part2(_: &Vec<String>) -> i32 {
    let weapons = vec![
        (8, 4, 0),
        (10, 5, 0),
        (25, 6, 0),
        (40, 7, 0),
        (74, 8, 0),
    ];

    let armors = vec![
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];

    let rings = vec![
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let mut max_cost: i32 = 0;

    for weapon in &weapons {
        for armor in &armors {
            for i in 0..rings.len() {
                for j in (i + 1)..rings.len() {
                    let ring1 = rings[i];
                    let ring2 = rings[j];
                    let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                    let damage = weapon.1 + ring1.1 + ring2.1;
                    let armor_value = armor.2 + ring1.2 + ring2.2;

                    if !boss_fight((100, damage, armor_value)) {
                        max_cost = max_cost.max(cost);
                    }
                }
            }
        }
    }

    max_cost
}