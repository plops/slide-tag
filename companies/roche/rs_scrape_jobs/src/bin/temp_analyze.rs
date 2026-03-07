use serde_json::Value;
use std::fs;

fn main() {
    for i in 1..=3 {
        let filename = format!("jobs_html/job_{}.json", i);
        println!("=== {} ===", filename);
        let content = fs::read_to_string(&filename).unwrap();
        let value: Value = serde_json::from_str(&content).unwrap();
        print_valuable_entries(&value, 0);
        println!();
    }
}

fn print_valuable_entries(v: &Value, depth: usize) {
    if depth > 4 {
        return;
    }
    if let Value::Object(map) = v {
        for (k, v) in map {
            match v {
                Value::String(s) if !s.is_empty() && s.len() < 100 => {
                    println!("{}- {}: \"{}\"", "  ".repeat(depth), k, s);
                }
                Value::Array(arr) if !arr.is_empty() => {
                    println!(
                        "{}- {}: [array with {} items]",
                        "  ".repeat(depth),
                        k,
                        arr.len()
                    );
                    if arr.len() <= 5 {
                        for (i, item) in arr.iter().enumerate() {
                            if let Value::String(s) = item {
                                println!("{}  [{}]: \"{}\"", "  ".repeat(depth), i, s);
                            }
                        }
                    }
                }
                Value::Object(_) => {
                    println!("{}- {}: {{object}}", "  ".repeat(depth), k);
                    if depth < 2 {
                        print_valuable_entries(v, depth + 1);
                    }
                }
                _ => {}
            }
        }
    }
}
