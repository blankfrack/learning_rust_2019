/// Simple test with standard library vector, Vec
pub fn test_vector() {
    println!("Testing Vectors");
    let mut vec = Vec::with_capacity(2); // We know the size that we need.  Otherwise, we could have used Vec::new()
    vec.push("Hello");
    vec.push("World");

    for s in vec {
        println!("{}", s);
    }

    // Use vec! macro to initialize Vec with the array syntax
    let v = vec![1, 2, 3];

    for i in v {
        println!("{}", i);
    }
}
