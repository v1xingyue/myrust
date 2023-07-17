use week5::keystore::Keystore;
use week5::network;

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
