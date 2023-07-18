use week5::{client, network};

#[tokio::main]
async fn main() {
    let network = network::default();
    let myclient = client::Client { network };
    let object_id =
        "0x2dfc31f14cc8b0040407e818568fa37e95e52281b684b67050ab32a16942d955".to_string();
    println!("gateway is : {}", myclient.network.get_gateway());
    println!("network is : {}", myclient.network);
    println!(
        "object link is : {} ",
        myclient.network.object_link(&object_id)
    );

    match myclient.get_object_id(&object_id).await {
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
