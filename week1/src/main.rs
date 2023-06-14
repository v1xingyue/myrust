mod game;
mod hello;
mod utils;
mod wrapper;

use hello::Hello;
use std::thread;
use std::time::Duration;

fn main() {
    game::main::start();
    let hello: Hello = Hello::new("hello world".to_string(), 12333333);
    Hello::say(hello);
    Hello::const_test();
    Hello::other_test();

    let mut wrapper = wrapper::types::new("hello message is inside box.".to_string());

    let data = wrapper::types::Wrapper::hello(&mut wrapper);
    println!("{}", data);

    let m: &mut String = wrapper::types::Wrapper::hello_mut(&mut wrapper);
    *m = String::from("this message chagned from outsice ...");

    println!(
        "now wrapper message is : {}",
        wrapper::types::Wrapper::get_data(&wrapper)
    );

    let mut d = Vec::new();
    Vec::push(&mut d, 1);
    Vec::push(&mut d, 2);
    Vec::push(&mut d, 3);

    for v in d.iter() {
        print!("current value: {} \n ", v);
    }

    let mut s: u64 = 0;
    let mut sum: u64 = s;

    loop {
        sum += s;
        if s >= 100 {
            break;
        } else {
            s += 1;
        }
    }

    println!("after loop sum is {}", sum);
    let start = utils::current_timestamp();

    loop {
        let now = utils::current_timestamp();
        println!("🚀 I am still running ... Current timestamp: {} 🚗 ", now);
        thread::sleep(Duration::from_secs(3));
        if (now - start) >= 15 {
            break;
        }
    }
}
