use week5::add;
use week5::api::sui::{example, VERSION};
use week5::utils::mark_line;

#[tokio::main]
async fn main() {
    mark_line(&"hello main start");
    println!("hello rust world!!! {}", add(3, 3));
    println!("api for sui vesion : {} ", VERSION);
    let result = example().await;
    println!("reult is : {:?}", result);
}
