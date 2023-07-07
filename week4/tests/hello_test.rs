use mylib::add;

#[path = "../src/myinfo.rs"]
mod myinfo;

#[test]
fn it_should_pass() {
    assert_ne!(add(100, 200), 320);
    assert_eq!(myinfo::hello_my_info("hello"), "hello");
    assert!(true);
    assert_ne!(1, 3);
    assert_eq!(1, 3);
}
