use std::collections::HashMap;

pub fn test_hashmaps() {
    println!("Testing HashMap");

    let mut h: HashMap<String, String> = HashMap::new();

    h.insert(String::from("Age"), String::from("42"));
    h.insert(String::from("Height"), String::from("5'8\""));
    h.insert(String::from("Weight"), String::from("185"));

    for i in 1..100 {
        h.insert(i.to_string(), i.to_string());
    }

    for s in h.iter() {
        println!("{} - {}", s.0, s.1);
    }

    println!("h.get(444) : {:#?}", h.get(&String::from("444")));
}
