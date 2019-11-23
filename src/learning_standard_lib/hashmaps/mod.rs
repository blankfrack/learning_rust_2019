use std::collections::HashMap;

pub fn test_hashmap() {
    println!("Test HashMap");

    let mut h: HashMap<String, String> = HashMap::new();

    h.insert(String::from("Age"), String::from("42"));
    h.insert(String::from("Height"), String::from("5'8\""));
    h.insert(String::from("Weight"), String::from("185"));

    for s in h.iter() {
        println!("{} - {}", s.0, s.1);
    }
}
