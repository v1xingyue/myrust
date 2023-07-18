use serde::{Deserialize, Serialize};
use serde_json::Value;
use week5::keystore::Keystore;
use week5::network;

enum BatchTransactionItem {
    TransferObject(TransferObjectParams),
    MoveCall(MoveCallParams),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferObjectParams {
    pub recipient: String,
    pub object_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveCallParams {
    pub package_object_id: String,
    pub module: String,
    pub function: String,
    #[serde(default)]
    pub type_arguments: Vec<String>,
    pub arguments: Vec<Value>,
}

#[tokio::main]
async fn main() {
    let network = network::network();
    let store = Keystore::default();
    println!("account count : {}", store.len());
    for idx in (0..=store.len() - 1).collect::<Vec<usize>>() {
        match store.load_account(idx) {
            Err(err) => println!("account load error : {}", err),
            Ok(account) => {
                println!("account address : {}", account.to_address());
                network.get_faucet(account.to_address()).await
            }
        }
    }
}
