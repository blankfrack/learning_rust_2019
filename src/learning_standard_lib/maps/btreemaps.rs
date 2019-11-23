use std::collections::BTreeMap;

pub fn test_btreemaps() {
    println!("Testing Maps");

    let mut map: BTreeMap<String, u8> = BTreeMap::new();

    map.insert(String::from("Quantity"), 89);
    map.insert(String::from("Count"), 33);
    map.insert(String::from("Quantity"), 100);

    // To prevent moving, we must use a reference to map (&map), otherwise we won't be able to call map.len()...
    for (s, i) in &map {
        println!("{} - {}", s, i);
    }

    println!("btreemap size : {}", map.len());
}
