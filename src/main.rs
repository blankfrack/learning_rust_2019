fn main() {
    test_move();

    test_pass_by_value(); 

    test_pass_by_reference(); 
}

fn test_move() {
    // Déclaration d'une variable
    let foo: i32 = 42;
    println!("foo : {}", foo); 

    let my_string: String = String::from("Hello World");
    println!("my_string : {}", my_string);

    {
        // Après le block, foo sera toujours accessible car type de base (implémente le trait Copy)
        let foo2 = foo;
        println!("foo dans le block : {}", foo);
        println!("foo2 dans le bloc : {}", foo2);

        // Move de variable. my_string ne sera plus accessible après le bloc car MOVED
        let my_string2 = my_string;

        // my_string a MOVED donc elle n'est plus accessible
        //print!("my_string : {}", my_string);
        println!("my_string2 : {}", my_string2);
    }

    // Impossible car on est sorti du bloc dans lequel foo2 a été déclarée
    //println!("foo2 après block : {}", foo2);

    // Toujours possible car foo est un type de base. Sa valeur est copié dans foo2 dans le bloc précédent
    //  foo conserve sa valeur
    println!("foo après le bloc : {}", foo);

    // Impossible.  La variable my_string n'est plus accessible car on la moved dans le bloc en la passant à my_string2
    //print!("my_string : {}", my_string);
}

fn test_pass_by_reference() {
    let my_string3: String = String::from("my_string3");

    // pass by reference. Keeps ownership
    print_string_reference(&my_string3);

    println!("my_string3 après call à print_string_reference : {}", my_string3);
}

fn test_pass_by_value() {
    let my_string4: String = String::from("my_string4");

    // pass by value and hence transfer ownership
    print_string_value(my_string4);

    // Impossible car moved
    //println!("my_string4 après call à print_string_value : {}", my_string4);
}


fn print_string_value(p_string: String) {
    println!("p_string : {}", p_string);
}

fn print_string_reference(p_string: &String) {
    println!("p_string : {}", p_string);
}
