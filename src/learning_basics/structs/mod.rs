pub struct Animal {
    name: String,
    age: u8,
}

pub fn test_structs() {
    println!("Structs");

    let animal: Animal = Animal {
        name: String::from("Fido"),
        age: 4,
    };

    println!("Animal name : {}, age : {}", animal.name, animal.age);
}
