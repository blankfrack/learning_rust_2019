// Module definitions here
mod learning_ownership;
mod learning_standard_lib;

// Define what we will be using
use learning_ownership::*;
use learning_standard_lib::hashmaps::*;
use learning_standard_lib::lists::linked_lists::*;
use learning_standard_lib::vectors::*;

/// Main
fn main() {
    test_move();

    test_pass_by_value();

    test_pass_by_reference();

    test_shared_borrow();

    test_vector();

    test_hashmap();

    test_linked_list();
}
