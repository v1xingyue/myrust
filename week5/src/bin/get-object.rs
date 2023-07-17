use week5::network::network;

#[tokio::main]
async fn main() {
    let network = network();
    let object_id =
        "0x2dfc31f14cc8b0040407e818568fa37e95e52281b684b67050ab32a16942d955".to_string();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);
    println!("object link is : {} ", network.object_link(&object_id));

    match network.get_object_id(&object_id).await {
        Ok(object) => {
            println!(
                "object detail : {}",
                serde_json::to_string_pretty(&object).unwrap()
            )
        }
        Err(err) => {
            println!("get object error : {}", err)
        }
    }
}
