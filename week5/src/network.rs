use crate::payload::{self, FilterOption, Payload};
use crate::response::{JsonResult, SimpleObject};
use crate::utils::CustomErr;
use reqwest::{self, Response};
use std::{env, error::Error, fmt::Display};

pub enum Network {
    Testnet,
    Mainnet,
    Devnet,
    Custom(String),
}

pub fn network() -> Network {
    const ENV_NAME: &str = "network";
    let mut network: Network = Network::Mainnet;
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

    pub fn faucet_url(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Network::Devnet => Ok("https://faucet.devnet.sui.io/gas".to_string()),
            Network::Testnet => Ok("https://faucet.testnet.sui.io/gas".to_string()),
            Network::Mainnet => Err(Box::new(CustomErr::new("mainnet does not support faucet"))),
            Network::Custom(url) => Ok(format!("{}/gas", url)),
        }
    }

    pub async fn get_faucet(&self, recipient: String) {
        let info = payload::new_faucet(recipient);
        println!(
            "send payload : {}",
            serde_json::to_string_pretty(&info).unwrap()
        );
        match self.faucet_url() {
            Err(err) => {
                println!("{}", err);
            }
            Ok(url) => {
                println!("faucet url : {}", url);
                let client = reqwest::Client::new();
                let resp = client
                    .post(url)
                    .header("Content-Type", "application/json")
                    .json(&info)
                    .send()
                    .await
                    .unwrap();
                println!("{}", resp.text().await.unwrap());
            }
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

    pub fn object_link(&self, object_id: &String) -> String {
        format!(
            "https://suiexplorer.com/object/{}?network={}",
            object_id,
            self.to_string()
        )
    }

    pub async fn sui_send_payload(&self, payload: &Payload) -> Result<Response, Box<dyn Error>> {
        println!("payload send : {}", payload);
        let gateway = self.get_gateway();
        let client = reqwest::Client::new();
        match client
            .post(gateway)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub async fn get_object_id(
        &self,
        object_id: &String,
    ) -> Result<JsonResult<SimpleObject>, Box<dyn Error>> {
        let payload: Payload = Payload::sui_get_object(object_id, &FilterOption::default_filter());
        let gateway = self.get_gateway();
        let client: reqwest::Client = reqwest::Client::new();
        match client
            .post(gateway)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Err(err) => Err(Box::new(err)),
            Ok(resp) => match resp.json::<JsonResult<SimpleObject>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
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
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.to_string(), self.get_gateway())
    }
}
