mod app;
mod callers;

fn main() {
    callers::starter::start_some_callers();
    callers::starter::start_some_callers_boxes();
    println!("hello world????");
}
