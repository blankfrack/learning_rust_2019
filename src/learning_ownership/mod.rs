/// Test moved
pub fn test_move() {
    // Déclaring a variable
    let foo: i32 = 42;
    println!("foo : {}", foo);

    let my_string: String = String::from("Hello World");
    println!("my_string : {}", my_string);

    {
        // After this block, foo will still be accessible bacause it's a base type (implementing Copy trait).  STACK
        let foo2 = foo;
        println!("foo in the block : {}", foo);
        println!("foo2 in the block : {}", foo2);

        // Variable moved ! my_string will not be accessible after this block.  HEAP
        let my_string2 = my_string;

        // my_string is MOVED so it's no longer accessible
        //print!("my_string : {}", my_string);
        println!("my_string2 : {}", my_string2);
    }

    // Impossible bacause we are now outside the block in which foo2 has been declared
    //println!("foo2 après block : {}", foo2);

    // Still possible because foo is a base type. Its value is copied into foo2 in the previous block
    //  foo keeps its value
    println!("foo after the block : {}", foo);

    // Impossible.  my_string is no longer accessible because it has been moved in the previous block to my_string2
    //print!("my_string : {}", my_string);
}

pub fn test_pass_by_reference() {
    let my_string3: String = String::from("my_string3");

    // pass by reference. Keeps ownership
    print_string_reference(&my_string3);

    println!(
        "my_string3 after calling print_string_reference : {}",
        my_string3
    );
}

pub fn test_pass_by_value() {
    let my_string4: String = String::from("my_string4");

    // pass by value and hence transfer ownership
    print_string_value(my_string4);

    // Impossible because moved
    //println!("my_string4 après call à print_string_value : {}", my_string4);
}

fn print_string_value(p_string: String) {
    println!("p_string : {}", p_string);
}

fn print_string_reference(p_string: &String) {
    println!("p_string : {}", p_string);
}

pub fn test_shared_borrow() {
    let mut name: String = String::from("Hello");
    name.push_str(" World!");

    print_name(&name);
    print_name(&name);
}

fn print_name(name: &String) {
    // Impossible.  Not mutable
    // name.push_str("string: &str");
    println!("{}", name);
}
