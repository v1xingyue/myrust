mod myerror;
use myerror::myerror::{assert_test, error_test, option_test};

fn main() {
    mylib::add(123, 355);

    option_test();
    error_test();
    // assert_test(0);
    assert_test(1, 1);
    assert_test(1, 5);
}
