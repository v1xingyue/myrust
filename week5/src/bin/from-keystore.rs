use week5::account::SuiAccount;
use week5::utils::base64_decode;

#[tokio::main]
async fn main() {
    let pair_from_keystore = "AAI9gSWWADI9gC6E53o1pfhaPSdhxNbQGjT6zTIjeijF";
    match base64_decode(pair_from_keystore) {
        Ok(data) => match SuiAccount::from_seed(&data[1..]) {
            Err(err) => {
                panic!("get account error : {}", err)
            }
            Ok(account) => {
                println!("account address is : {}", account.to_address());
                assert_eq!(
                    account.to_address(),
                    "0x0a27f6f7d3b7907fbcc4265ee8e63f5447312a8f53fb270a36f892e6f264008f"
                )
            }
        },
        Err(err) => {
            panic!("err {}", err)
        }
    }
}
