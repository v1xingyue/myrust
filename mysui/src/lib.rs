use http_req::{
    request::{Method, Request},
    uri::Uri,
};

pub fn example_sui_request() {
    let mut writer = Vec::new(); // Used to store the server response
    let body = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sui_getObject",
        "params": [
            "0x99c989edd18b84ff547720682ddaf5cdc2dfc48c2d6655894a2e374394266b76",
            {
                "showType": true,
                "showOwner": true,
                "showPreviousTransaction": true,
                "showDisplay": false,
                "showContent": true,
                "showBcs": false,
                "showStorageRebate": true
            }
        ]
    }"#;
    const SUI_RPC: &str = "https://fullnode.mainnet.sui.io:443/";
    let uri = Uri::try_from(SUI_RPC).unwrap();

    let response = Request::new(&uri)
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Content-Length", &body.len())
        .body(body.as_bytes())
        .send(&mut writer);

    match response {
        Ok(response) => {
            println!("Response: {:?}", response);
            println!("Body: \n {} \n", String::from_utf8(writer).unwrap());
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_sui_get_object() {
        example_sui_request();
    }
}
