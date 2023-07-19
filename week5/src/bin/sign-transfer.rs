use week5::{client, keystore::Keystore, network};

#[tokio::main]
async fn main() {
    let network = network::default();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);
    let myclient = client::default_client(network);
    let (owner_address, object_id, gas_object, gas_budget, to_address) = (
        "0x0a27f6f7d3b7907fbcc4265ee8e63f5447312a8f53fb270a36f892e6f264008f",
        "0x104732c4b8961870be54b9d04c33cb54dfec72574c33aa0cce640e6dbfb56756",
        "0xcea9e5f61d0ea45058e90fae2b6422ebbbafb8c31ad01f263ec45b06e3eaf7df",
        3000_000,
        "0x0a27f6f7d3b7907fbcc4265ee8e63f5447312a8f53fb270a36f892e6f264008f",
    );

    match myclient
        .unsafe_transfer_object(owner_address, object_id, gas_object, gas_budget, to_address)
        .await
    {
        Err(err) => {
            println!("{}", err)
        }
        Ok(result) => {
            println!(
                "unsigned result : {}",
                serde_json::to_string(&result).unwrap()
            );

            let store = Keystore::default();
            let account = store.load_account(0).unwrap();
            let signed_transaction = account.sign_unsafe_transaciton(result.result);
            match myclient.send_payload_effect(&signed_transaction).await {
                Err(err) => {
                    println!("signed result error : {}", err)
                }
                Ok(data) => {
                    println!("reuslt : {}", serde_json::to_string_pretty(&data).unwrap());
                    println!(
                        "transaction link : {}",
                        myclient
                            .network
                            .transaction_link(&data.result.digest.to_string())
                    )
                }
            }
        }
    }
}
