// Module definitions here
mod learning_basics;
mod learning_ownership;
mod learning_standard_lib;

// Define what we will be using
use learning_basics::structs::*;
use learning_ownership::*;
use learning_standard_lib::arrays::*;
use learning_standard_lib::lists::linked_lists::*;
use learning_standard_lib::maps::{btreemaps::*, hashmaps::*};
use learning_standard_lib::vectors::*;

/// Main
fn main() {
    println!("Learning the basics");
    test_structs();
    test_pass_by_value();
    test_pass_by_reference();
    test_shared_borrow();

    println!("Learning Ownership");
    test_move();

    println!("Learning the Standard Library");
    println!("Learning collections");

    test_arrays();
    test_vector();
    test_hashmaps();
    test_btreemaps();
    test_linked_list();
}
