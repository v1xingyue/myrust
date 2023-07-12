pub mod sui {

    use reqwest;
    use serde::{Deserialize, Serialize};
    use serde_json::{to_value, Value};
    use std::env;
    use std::fmt::Display;

    use crate::utils::current_timestamp;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct FilterOption {
        show_type: bool,
        show_owner: bool,
        show_previous_transaction: bool,
        show_display: bool,
        show_content: bool,
        show_bcs: bool,
        show_storage_rebate: bool,
    }

    impl FilterOption {
        pub fn default_filter() -> Self {
            Self {
                show_type: true,
                show_owner: true,
                show_previous_transaction: true,
                show_display: (false),
                show_content: (true),
                show_bcs: (false),
                show_storage_rebate: (false),
            }
        }

        pub fn new_filter(
            show_type: bool,
            show_owner: bool,
            show_previous_transaction: bool,
            show_display: bool,
            show_content: bool,
            show_bcs: bool,
            show_storage_rebate: bool,
        ) -> Self {
            Self {
                show_type,
                show_owner,
                show_previous_transaction,
                show_display,
                show_content,
                show_bcs,
                show_storage_rebate,
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Payload {
        jsonrpc: String,
        id: u64,
        method: String,
        params: Vec<Value>,
    }

    impl Payload {
        pub fn method_paylod(method: &str) -> Self {
            Self {
                jsonrpc: String::from(PAYLOAD_JSONRPC_VERSION),
                id: current_timestamp(),
                method: String::from(method),
                params: vec![],
            }
        }

        pub fn sui_get_object(id: &str, option: &FilterOption) -> Self {
            Self {
                jsonrpc: String::from(PAYLOAD_JSONRPC_VERSION),
                id: current_timestamp(),
                method: String::from("sui_getObject"),
                params: vec![Value::String(id.to_string()), to_value(option).unwrap()],
            }
        }
    }

    pub enum Network {
        Testnet,
        Mainnet,
        Devnet,
        Custom(String),
    }

    pub fn network() -> Network {
        const ENV_NAME: &str = "network";
        let mut network = Network::Mainnet;
        if env::var_os(ENV_NAME).is_some() {
            if let Ok(value) = env::var(ENV_NAME) {
                if value.eq("testnet") {
                    network = Network::Testnet;
                } else if value.eq("devnet") {
                    network = Network::Devnet;
                } else if value.eq("mainnet") {
                    network = Network::Mainnet;
                } else {
                    network = Network::Custom(value);
                }
            }
        }
        network
    }

    impl Network {
        pub fn get_gateway(&self) -> String {
            match self {
                Network::Testnet => String::from("https://fullnode.testnet.sui.io:443"),
                Network::Mainnet => String::from("https://fullnode.mainnet.sui.io:443"),
                Network::Devnet => String::from("https://fullnode.devnet.sui.io:443"),
                Network::Custom(url) => url.clone(),
            }
        }
        pub fn to_string(&self) -> String {
            match self {
                Network::Testnet => String::from("testnet"),
                Network::Mainnet => String::from("mainnet"),
                Network::Devnet => String::from("devnet"),
                Network::Custom(url) => url.clone(),
            }
        }

        pub async fn get_object_id(&self, id: &String) {
            let payload = Payload::sui_get_object(id, &FilterOption::default_filter());
            let gateway = self.get_gateway();
            let client = reqwest::Client::new();
            let res = client
                .post(gateway)
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await;
            match res {
                Err(err) => {
                    println!("error: {}", err);
                }
                Ok(resp) => {
                    println!("{:?}", resp.text().await)
                }
            }
        }
    }

    impl Display for Network {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.to_string(), self.get_gateway())
        }
    }

    pub const VERSION: &str = "0.0.0";
    pub const PAYLOAD_JSONRPC_VERSION: &str = &"2.0";

    pub async fn example() {
        let payload = Payload {
            jsonrpc: String::from(PAYLOAD_JSONRPC_VERSION),
            id: current_timestamp(),
            method: String::from("get"),
            params: vec![
                Value::String(
                    "0x70f2d83f980fe0996a92d351d6749a0a0b47998aaf3d11da24c49e014d4ccb1d"
                        .to_string(),
                ),
                Value::Number(3.into()),
            ],
        };
        let client = reqwest::Client::new();
        let res = client
            .post("http://httpbin.org/post")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;
        match res {
            Err(err) => println!("error : {}", err),
            Ok(resp) => {
                println!("status : {}", resp.status());
                match resp.text().await {
                    Err(err) => {
                        println!("text read error : {}", err)
                    }
                    Ok(data) => {
                        println!("{}", data);
                    }
                }
            }
        }
    }
}
