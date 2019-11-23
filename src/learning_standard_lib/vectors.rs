/// Simple test with standard library vector, Vec
pub fn test_vector() {
    let mut vec = Vec::new();
    vec.push("Hello");
    vec.push("World");

    for s in vec {
        println!("{}", s);
    }
}
