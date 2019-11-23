use std::collections::LinkedList;

pub fn test_linked_list() {
    println!("Test LinkedList");

    let mut l: LinkedList<u32> = LinkedList::new();

    l.push_back(42);
    l.push_back(55);
    l.push_back(78);
    l.push_back(100);
    l.push_back(543);

    // To prevent moving, we must use a reference to l (&l), otherwise we won't be able to call l.pop_back() etc...
    for i in &l {
        println!("{}", i);
    }

    l.pop_back();
    println!("Length after pop_back : {}", l.len());
}
