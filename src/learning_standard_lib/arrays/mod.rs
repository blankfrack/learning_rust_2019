pub fn test_arrays() {
    println!("Testing Arrays");
    println!("Declaring an array of i32 of 3 elements");
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    // This loop prints: 0 1 2
    for x in &array {
        println!("{} ", x);
    }
}
