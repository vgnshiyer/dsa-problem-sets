#[allow(dead_code)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl Ingredient {
    fn new(name: String, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Ingredient {
        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

fn store_ingredients(ingredient_list: &Vec<String>, ingredients: &mut Vec<Ingredient>) {
    for ingredient in ingredient_list {
        let parts: Vec<&str> = ingredient.split_whitespace().collect();
        let name: String = parts[0].trim_end_matches(":").to_string();
        let capacity: i32 = parts[2].trim_end_matches(",").parse().unwrap();
        let durability: i32 = parts[4].trim_end_matches(",").parse().unwrap();
        let flavor: i32 = parts[6].trim_end_matches(",").parse().unwrap();
        let texture: i32 = parts[8].trim_end_matches(",").parse().unwrap();
        let calories: i32 = parts[10].parse().unwrap();
        ingredients.push(Ingredient::new(name, capacity, durability, flavor, texture, calories));
    }
}

pub fn part1(ingredient_list: &Vec<String>) -> u64 {
    let mut total_score: u64 = 0;
    let mut ingredients: Vec<Ingredient> = Vec::new();
    store_ingredients(ingredient_list, &mut ingredients);
    
    for i in 0..=100 {
        for j in 0..=100-i {
            for k in 0..100-i-j {
                let l = 100 - i - j - k;
                let capacity: i32 = std::cmp::max(0, i * ingredients[0].capacity + j * ingredients[1].capacity + k * ingredients[2].capacity + l * ingredients[3].capacity);
                let durability: i32 = std::cmp::max(0, i * ingredients[0].durability + j * ingredients[1].durability + k * ingredients[2].durability + l * ingredients[3].durability);
                let flavor: i32 = std::cmp::max(0, i * ingredients[0].flavor + j * ingredients[1].flavor + k * ingredients[2].flavor + l * ingredients[3].flavor);
                let texture: i32 = std::cmp::max(0, i * ingredients[0].texture + j * ingredients[1].texture + k * ingredients[2].texture + l * ingredients[3].texture);
                
                let score: u64 = (capacity * durability * flavor * texture).try_into().unwrap();
                total_score = total_score.max(score);
            }
        }
    }
    total_score
}

pub fn part2(ingredient_list: &Vec<String>) -> u64 {
    let mut total_score: u64 = 0;
    let mut ingredients: Vec<Ingredient> = Vec::new();
    store_ingredients(ingredient_list, &mut ingredients);

    for i in 0..=100 {
        for j in 0..=100-i {
            for k in 0..100-i-j {
                let l = 100 - i - j - k;
                let capacity: i32 = std::cmp::max(0, i * ingredients[0].capacity + j * ingredients[1].capacity + k * ingredients[2].capacity + l * ingredients[3].capacity);
                let durability: i32 = std::cmp::max(0, i * ingredients[0].durability + j * ingredients[1].durability + k * ingredients[2].durability + l * ingredients[3].durability);
                let flavor: i32 = std::cmp::max(0, i * ingredients[0].flavor + j * ingredients[1].flavor + k * ingredients[2].flavor + l * ingredients[3].flavor);
                let texture: i32 = std::cmp::max(0, i * ingredients[0].texture + j * ingredients[1].texture + k * ingredients[2].texture + l * ingredients[3].texture);
                let calories: i32 = std::cmp::max(0, i * ingredients[0].calories + j * ingredients[1].calories + k * ingredients[2].calories + l * ingredients[3].calories);

                let score: u64 = (capacity * durability * flavor * texture).try_into().unwrap();
                if calories == 500 {
                    total_score = total_score.max(score);
                }
            }
        }
    }
    total_score
}