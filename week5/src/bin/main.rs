use week5::add;
use week5::api::sui::{example, network, VERSION};
use week5::utils::mark_line;

#[tokio::main]
async fn main() {
    mark_line(&"hello main start");
    println!("hello rust world!!! {}", add(3, 3));
    println!("api for sui vesion : {} ", VERSION);
    let result = example().await;
    println!("reult is : {:?}", result);

    let network = network();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);

    network
        .get_object_id(
            &"0x2dfc31f14cc8b0040407e818568fa37e95e52281b684b67050ab32a16942d955".to_string(),
        )
        .await
}
