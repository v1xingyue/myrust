pub mod sui {

    use reqwest;
    use serde::{Deserialize, Serialize};
    use serde_json::{to_value, Value};
    use std::env;
    use std::fmt::Display;

    use crate::utils::current_timestamp;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct TransactionBlockResponseOptions {
        show_input: bool,
        show_raw_input: bool,
        show_effects: bool,
        show_events: bool,
        show_object_changes: bool,
        show_balance_changes: bool,
    }

    impl TransactionBlockResponseOptions {
        pub fn new(
            show_input: bool,
            show_raw_input: bool,
            show_effects: bool,
            show_events: bool,
            show_object_changes: bool,
            show_balance_changes: bool,
        ) -> Self {
            Self {
                show_input,
                show_raw_input,
                show_effects,
                show_events,
                show_object_changes,
                show_balance_changes,
            }
        }

        pub fn default_options() -> Self {
            Self::new(true, true, true, true, true, true)
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FilterOption {
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
            Self::new(true, true, true, false, true, false, false)
        }

        pub fn new(
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
    pub struct Payload {
        jsonrpc: String,
        id: u64,
        method: String,
        params: Vec<Value>,
    }

    impl Display for Payload {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "payload is : {}", serde_json::to_string(self).unwrap())
        }
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

        pub fn sui_get_object(object_id: &str, option: &FilterOption) -> Self {
            Self {
                jsonrpc: String::from(PAYLOAD_JSONRPC_VERSION),
                id: current_timestamp(),
                method: String::from("sui_getObject"),
                params: vec![
                    Value::String(object_id.to_string()),
                    to_value(option).unwrap(),
                ],
            }
        }

        pub fn unsafe_transfer_object(
            owner_address: &str,
            object_id: &str,
            gas_object: &str,
            gas_budget: u64,
            to_address: &str,
        ) -> Self {
            Self {
                jsonrpc: String::from(PAYLOAD_JSONRPC_VERSION),
                id: current_timestamp(),
                method: String::from("unsafe_transferObject"),
                params: vec![
                    Value::String(owner_address.to_string()),
                    Value::String(object_id.to_string()),
                    Value::String(gas_object.to_string()),
                    Value::String(format!("{}", gas_budget)),
                    Value::String(to_address.to_string()),
                ],
            }
        }

        pub fn transaction_block_payload(tx_bytes: &str, signatures: &str) -> Self {
            let option = TransactionBlockResponseOptions::default_options();
            Self {
                jsonrpc: String::from(PAYLOAD_JSONRPC_VERSION),
                id: current_timestamp(),
                method: String::from("sui_executeTransactionBlock"),
                params: vec![
                    Value::String(tx_bytes.to_string()),
                    Value::Array(vec![Value::String(signatures.to_string())]),
                    to_value(&option).unwrap(),
                    Value::String("WaitForLocalExecution".to_string()),
                ],
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

        pub async fn unsafe_transfer_object(
            &self,
            owner_address: &str,
            object_id: &str,
            gas_object: &str,
            gas_budget: u64,
            to_address: &str,
        ) {
            let payload: Payload = Payload::unsafe_transfer_object(
                owner_address,
                object_id,
                gas_object,
                gas_budget,
                to_address,
            );

            println!("{}", payload);

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
                Ok(resp) => match resp.text().await {
                    Ok(body) => println!("{}", body),
                    Err(err) => println!("{}", err),
                },
            }
        }

        pub async fn sui_send_payload(&self, payload: &Payload) {
            println!("{}", payload);

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
                Ok(resp) => match resp.text().await {
                    Ok(body) => println!("{}", body),
                    Err(err) => println!("{}", err),
                },
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
