use week5::{network::network, payload::Payload};

#[tokio::main]
async fn main() {
    let network = network();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);
    let tx_bytes = "AAACACAKJ/b307eQf7zEJl7o5j9URzEqj1P7Jwo2+JLm8mQAjwEAxeH0cqZyHhw5O9ex6npRVN/VqjeaklGk0sd3652k4IBGAAAAAAAAACAhCZTCQGadfZFHMUOmF/7vzYjaOL3iOFOttgQ8Vq8WRwEBAQEBAAEAAAon9vfTt5B/vMQmXujmP1RHMSqPU/snCjb4kubyZACPASyl4AYgixH+XTi5XBSI10IUmYMOMKQxedmoPg/qzXfZRQAAAAAAAAAgbcf/hvgkTrSAFqX06JcGyUca6ZeqRPOSOhgE/MNEw88KJ/b307eQf7zEJl7o5j9URzEqj1P7Jwo2+JLm8mQAj+gDAAAAAAAAwMYtAAAAAAAA";
    // sui keytool sign --address <owner_address> --data <tx_bytes>
    // 取  Serialized signature
    let signatures = "ABCbWyMJdo/y+RDUSqJ0TghGwzfQbmVTYHdb/FQ9SX3YybVkRrB+6nh4qutm7E1ZRqUzzC0YiG2FY9rl5IQkNAewlwaDbsn0alvR1qMy7xdd9548ZGz4MI7Mp0lic5Scsg==";
    let payload = Payload::safe_transaction_block_payload(tx_bytes, signatures);
    match network.send_payload_effect(&payload).await {
        Err(err) => {
            println!("send error : {}", err)
        }
        Ok(data) => {
            println!("data return : {}", data.result.digest)
        }
    }
}
