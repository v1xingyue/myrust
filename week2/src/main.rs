mod lifecycle;
mod memory_use;
mod owner;
fn main() {
    let a = lifecycle::hello(&3);
    println!("value of a is : {}", a);

    lifecycle::hello_lifecycle(&1, &2, &3);
    lifecycle::hello_3(&3, &4);

    memory_use::example::hello();
    owner::owner_test();
    memory_use::example::memory_usage_test();
}
