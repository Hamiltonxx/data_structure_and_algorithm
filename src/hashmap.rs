use std::collections::HashMap;

fn main() {
    let mut map:HashMap<&str,i32> = HashMap::new();

    map.insert("apple", 3);
    map.insert("banana", 5);

    match map.get("apple") {
        Some(&count) => println!("苹果的数量: {count}"),
        None => println!("没有找到苹果"),
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    map.remove("banana");
    println!("{:?}", map);

    if let Some(&count) = map.get("apple") { // &str
        println!("apple count: {count}");
    }
}
