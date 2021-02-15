use serde_json::Value;
use std::fs;

fn sum(v: Value, skip_red: bool) -> i64 {
    match v {
        Value::Array(values) => values.into_iter().map(|v| sum(v, skip_red)).sum(),
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Object(m) => {
            if skip_red && m.values().any(|k| k == "red") {
                0
            } else {
                m.into_iter().map(|(_, v)| sum(v, skip_red)).sum()
            }
        }
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("aoc2015/inputs/day12.input").unwrap();
    let v: Value = serde_json::from_str(&input).unwrap();
    println!("part 1: {}", sum(v.clone(), false));
    println!("part 2: {}", sum(v, true));
}
