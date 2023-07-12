use week5::add;
use week5::api::sui::{example, network, Payload, VERSION};
use week5::utils::mark_line;

#[tokio::main]
async fn main() {
    mark_line(&"hello main start");
    println!("hello rust world!!! {}", add(3, 3));
    println!("api for sui vesion : {} ", VERSION);
    let result = example().await;
    println!("reult is : {:?}", result);

    let network = network();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);

    network
        .get_object_id(
            &"0x2dfc31f14cc8b0040407e818568fa37e95e52281b684b67050ab32a16942d955".to_string(),
        )
        .await;

    mark_line("do transfer objtect test");
    println!("send request to : {}", network);
    network
        .unsafe_transfer_object(
            "0xd55b5299b15596e6b29547629c12a02273ccfa2eeb398b05f17313d12267e3e2",
            "0x1d37b08a7f66467f35027ed16b5ce03eaac31d43b99d37d7cc3b5af8246ae52e",
            "0x73d1b52e10daf37c5ebe74d97f05da9a0bd626c35b38e38f06b530990fc16ec8",
            2000000,
            "0xc4d17bdea567268b50cb24c783ccafc678d468a0cfce0afb84313b163cb403ef",
        )
        .await;

    mark_line("execute transaction block ");
    let tx_bytes = "AAACACDE0XvepWcmi1DLJMeDzK/GeNRooM/OCvuEMTsWPLQD7wEAHTewin9mRn81An7Ra1zgPqrDHUO5nTfXzDta+CRq5S4cAAAAAAAAACDRB03SvA8oBVsmiEygfvtTSKRXDW3XDmjgBJvoWmgJsgEBAQEBAAEAANVbUpmxVZbmspVHYpwSoCJzzPou6zmLBfFzE9EiZ+PiAXPRtS4Q2vN8Xr502X8F2poL1ibDWzjjjwa1MJkPwW7IGQAAAAAAAAAgCz2+u5h8fLl1qx+++s0h4DdHY1CKbiRAw/kRBG68+8fVW1KZsVWW5rKVR2KcEqAic8z6Lus5iwXxcxPRImfj4ugDAAAAAAAAgIQeAAAAAAAA";
    let signatures = "AKPwdX6pO5ADSygMfXyYhV2NDS0ItL2OMGCvjzztB5/4rE8KhpbMd7KdVVDrIDicCIGuswfLgt0zI4AwZGQf1QWswVmKtbvTIPFtE8TxtZv503JaWlLcg5DkdWfZPQhRkg==";
    let payload = Payload::transaction_block_payload(tx_bytes, signatures);

    network.sui_send_payload(&payload).await;
}
