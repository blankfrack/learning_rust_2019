/// Test moved
pub fn test_move() {
    // Déclaring a variable
    let foo: i32 = 42;
    println!(
        "Declaring an immutable i32 variable (on the STACK) named foo with value {}",
        foo
    );

    let my_string: String = String::from("Hello World");
    println!(
        "Declaring an immutable String variable (on the HEAP) named my_string with value {}",
        my_string
    );

    {
        // After this block, foo will still be accessible because it's a base type (implementing Copy trait).  STACK
        let foo2 = foo;
        println!("In a block, assign foo to foo2.");
        println!("foo  : {}", foo);
        println!("foo2  : {}", foo2);

        // Variable moved ! my_string will not be accessible after this block.  HEAP
        let my_string2 = my_string;

        // my_string is MOVED so it's no longer accessible
        println!("In the same block, assign my_string to my_string2");
        print!("* my_string is has been moved and is no longer accessible");
        //print!("my_string : {}", my_string);
        println!("my_string2 : {}", my_string2);
        println!("The block ends here.");
    }

    // Impossible because we are now outside the block in which foo2 has been declared
    println!("*Outside the block, foo2 is no longer available.");
    //println!("foo2 après block : {}", foo2);

    // Still possible because foo is a base type. Its value is copied into foo2 in the previous block
    //  foo keeps its value
    println!("However, foo is still available.  foo : {}", foo);

    // Impossible.  my_string is no longer accessible because it has been moved to my_string2 in the previous block
    println!("*Outside the block, my_string is no longer available because it has been moved to my_string2 in the block")
    //print!("my_string : {}", my_string);
}

/// Test Passed by Reference
pub fn test_pass_by_reference() {
    let my_string3: String = String::from("my_string3");
    println!("In test_pass_by_reference, declaring a variable my_string3");

    // pass by reference. Keeps ownership
    println!("Passing my_string3 reference to function print_string_reference(&my_string3)");
    print_string_reference(&my_string3);
    println!(
        "In test_pass_by_reference, my_string3 after calling print_string_reference : {}",
        my_string3
    );
}

/// Test Passed by Value
pub fn test_pass_by_value() {
    let my_string4: String = String::from("my_string4");

    // pass by value and hence transfer ownership
    print_string_value(my_string4);

    // Impossible because moved
    //println!("my_string4 après call à print_string_value : {}", my_string4);
}

/// Printing a string received by value
fn print_string_value(p_string: String) {
    println!("p_string : {}", p_string);
}

/// Printing a string received by reference
fn print_string_reference(p_string: &String) {
    println!("p_string : {}", p_string);
}

/// Test Shared Barrow
pub fn test_shared_borrow() {
    let mut name: String = String::from("Hello");
    name.push_str(" World!");

    print_name(&name);
    print_name(&name);
}

/// Printing a name received by reference
fn print_name(name: &String) {
    // Impossible.  Not mutable
    // name.push_str("string: &str");
    println!("{}", name);
}
