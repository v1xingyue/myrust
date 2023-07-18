use crate::network::Network;
use crate::payload::{self, FilterOption, Payload};
use crate::response::{JsonResult, SimpleObject, TransactionEffect, UnsafeTransactionResult};
use reqwest::{self, Response};
use serde_json::Value;
use std::error::Error;

pub struct Client {
    pub network: Network,
}

impl Client {
    pub async fn get_faucet(&self, recipient: String) {
        let info = payload::new_faucet(recipient);
        println!(
            "send payload : {}",
            serde_json::to_string_pretty(&info).unwrap()
        );
        match self.network.faucet_url() {
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

    pub async fn send_payload_effect(
        &self,
        payload: &Payload,
    ) -> Result<JsonResult<TransactionEffect>, Box<dyn Error>> {
        println!("payload send : {}", payload);
        let gateway = self.network.get_gateway();
        let client = reqwest::Client::new();
        match client
            .post(gateway)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<JsonResult<TransactionEffect>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
            Err(err) => Err(Box::new(err)),
        }
    }

    pub async fn sui_send_payload(&self, payload: &Payload) -> Result<Response, Box<dyn Error>> {
        println!("payload send : {}", payload);
        let gateway = self.network.get_gateway();
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
        let gateway = self.network.get_gateway();
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
    ) -> Result<JsonResult<UnsafeTransactionResult>, Box<dyn Error>> {
        let payload: Payload = Payload::unsafe_transfer_object(
            owner_address,
            object_id,
            gas_object,
            gas_budget,
            to_address,
        );

        let gateway = self.network.get_gateway();
        println!("payload content : {} ", payload);
        let client = reqwest::Client::new();
        let res = client
            .post(gateway)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;
        match res {
            Err(err) => Err(Box::new(err)),
            Ok(resp) => match resp.json::<JsonResult<UnsafeTransactionResult>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn unsafe_move_call(
        &self,
        owner_address: &str,
        package_object_id: &str,
        module: &str,
        function: &str,
        type_arguments: Vec<Value>,
        arguments: Vec<Value>,
        gas_object: &str,
        gas_budget: u64,
    ) -> Result<JsonResult<UnsafeTransactionResult>, Box<dyn Error>> {
        let payload: Payload = Payload::move_call(
            owner_address,
            package_object_id,
            module,
            function,
            type_arguments,
            arguments,
            gas_object,
            gas_budget,
        );

        let gateway = self.network.get_gateway();
        println!("payload content : {} ", payload);
        let client = reqwest::Client::new();
        let res = client
            .post(gateway)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;
        match res {
            Err(err) => Err(Box::new(err)),
            Ok(resp) => match resp.json::<JsonResult<UnsafeTransactionResult>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }
}
