use std::vec;

use week5::{keystore::Keystore, network::network, payload::Payload};

#[tokio::main]
async fn main() {
    let current_network = network();
    println!("gateway is : {}", current_network.get_gateway());
    println!("network is : {}", current_network);

    let store = Keystore::default();
    let account = store.load_account(0).unwrap();

    let payload = Payload::move_call(
        &account.to_address(),
        &"0x988fb71f38bb0323eeb5014c7a00e5988b047c09f39d58f157fc67d43ddfc091",
        &"hello_world",
        "mint",
        vec![],
        vec![],
        &"0x6abb224a86b8e571f221ea6bf6a5028923b29b13201a3c29f6fdaaaa3b4cbb97",
        3000_000,
    );

    match current_network.sui_send_payload(&payload).await {
        Err(err) => {
            println!("{}", err)
        }
        Ok(result) => {
            println!("unsigned result : {}", result.text().await.unwrap())
        }
    };

    match current_network
        .unsafe_move_call(
            &account.to_address(),
            &"0x988fb71f38bb0323eeb5014c7a00e5988b047c09f39d58f157fc67d43ddfc091",
            &"hello_world",
            "mint",
            vec![],
            vec![],
            &"0x6abb224a86b8e571f221ea6bf6a5028923b29b13201a3c29f6fdaaaa3b4cbb97",
            3000_000,
        )
        .await
    {
        Err(err) => {
            println!("{}", err)
        }
        Ok(data) => {
            let signed_payload = account.sign_unsafe_transaciton(data.result);

            let effet = current_network
                .send_payload_effect(&signed_payload)
                .await
                .unwrap();

            println!("reuslt : {}", serde_json::to_string_pretty(&effet).unwrap());
            println!(
                "transaction link : {}",
                current_network.transaction_link(&effet.result.digest.to_string())
            )
        }
    }
}
