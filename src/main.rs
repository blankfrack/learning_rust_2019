// On défini les modules
mod learning_ownership;
mod learning_standard_lib;

// On défini ce que nous allons utiliser dans les modules/librairies standard/librairie importées
use learning_ownership::*;
use learning_standard_lib::vectors::*;

/// Main
fn main() {
    test_move();

    test_pass_by_value();

    test_pass_by_reference();

    test_shared_borrow();

    test_vector();
}
