use std::collections::LinkedList;

pub fn test_linked_list() {
    println!("Test LinkedList");

    let mut l : LinkedList<u32> = LinkedList::new();

    l.push_back(42);
    l.push_back(55);

    for i in l {
        println!("{}", i);
    }
}