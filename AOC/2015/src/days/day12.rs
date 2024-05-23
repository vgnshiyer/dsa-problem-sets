use serde_json::Value;

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

pub fn part1(json: &str) -> i32 {
    let chars: Vec<char> = json.chars().collect();
    let mut total: i32 = 0;
    let mut current_number: String = String::new();
    for c in chars {
        if c == '-' || is_number(&c.to_string()) {
            current_number += &c.to_string();
        } else {
            if !current_number.is_empty() {
                total += current_number.parse::<i32>().unwrap();
            }
            current_number = "".to_string();
        }
    }
    total
}

#[allow(dead_code)]
fn simplify(json: serde_json::Value) -> i32 {
    let mut total: i32 = 0;
    if let Some(obj) = json.as_object() {
        for value in obj.values() {
            if let Some(string) = value.as_str() {
                if string == "red" {
                    return 0;
                }
            } else if let Some(number) = value.as_i64() {
                total += number as i32;
            } else if value.is_object() {
                total += simplify(value.clone());
            } else if value.is_array() {
                total += simplify(value.clone());
            }
        }
    } else if let Some(arr) = json.as_array() {
        for item in arr {
            if let Some(number) = item.as_i64() {
                total += number as i32;
            } else if item.is_object() {
                total += simplify(item.clone());
            } else if item.is_array() {
                total += simplify(item.clone());
            }
        }
    } else if let Some(number) = json.as_i64() {
        total += number as i32;
    }
    total
}

fn simplify_with_swag(json: serde_json::Value) -> i32 {
    match json {
        Value::Object(obj) => {
            for value in obj.values() {
                if value.as_str() == Some("red") {
                    return 0;
                }
            }
            obj.values().into_iter().map(|v| simplify_with_swag(v.clone())).sum()
        },
        Value::Array(arr) => arr.into_iter().map(simplify_with_swag).sum(),
        Value::Number(num) => num.as_i64().unwrap() as i32,
        _ => 0
    }
}

pub fn part2(json: &str) -> i32 {
    let json_data: Value = serde_json::from_str(json).unwrap();
    // simplify(json_data)
    simplify_with_swag(json_data)
}
