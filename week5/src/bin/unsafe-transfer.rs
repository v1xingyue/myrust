use week5::network::network;

#[tokio::main]
async fn main() {
    let network = network();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);
    let (owner_address, object_id, gas_object, gas_budget, to_address) = (
        "0x0a27f6f7d3b7907fbcc4265ee8e63f5447312a8f53fb270a36f892e6f264008f",
        "0xc5e1f472a6721e1c393bd7b1ea7a5154dfd5aa379a9251a4d2c777eb9da4e080",
        "0x2ca5e006208b11fe5d38b95c1488d7421499830e30a43179d9a83e0feacd77d9",
        3000_000,
        "0x0a27f6f7d3b7907fbcc4265ee8e63f5447312a8f53fb270a36f892e6f264008f",
    );
    match network
        .unsafe_transfer_object(owner_address, object_id, gas_object, gas_budget, to_address)
        .await
    {
        Err(err) => {
            println!("{}", err)
        }
        Ok(result) => {
            println!(
                "object detail : {}",
                serde_json::to_string_pretty(&result).unwrap()
            )
        }
    }
}
