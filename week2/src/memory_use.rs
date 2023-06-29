pub mod example {
    use std::thread;
    use std::time::Duration;

    pub const SUI_RPC_HOST: &str = "https://fullnode.testnet.sui.io:443";

    pub fn hello() {
        week1::mark_line("start myrpc tester");
        println!("sui host is : {}", SUI_RPC_HOST);
        let pair = week1::get_keypair();
        let address = week1::to_address(&pair.public);
        println!("address is {}", address);
        week1::sui_signer_validator(b"hello this is big world", "AB7g17GZ0VIL1n3Bz+nTBW/PNlk39LANbhpPN93FupZYSpdtSnAOJFDZhD+RqWvNyxy3yQJN3j7Ukzn3TaLTxA1zboeZV/gAhZgweIWbU+ezTPxHUe9y1fiRw7inQf9suw==".to_string());
    }

    pub fn memory_usage_test() {
        {
            let mut big_string = String::from("hello world");
            let mut append_times: u64 = 30;
            while append_times > 0 {
                let mut sub_loop = 0;
                while sub_loop <= 1000000 {
                    big_string.push_str("another block of big memory another block of big memory another block of big memory another block of big memory ");
                    sub_loop += 1;
                }

                append_times -= 1;
                thread::sleep(Duration::from_secs(1));
                println!("I am running");
            }

            let big_string = String::from("new value of..");
            println!("memory usage done .... {} 000000 ", big_string);
            thread::sleep(Duration::from_secs(15));
        }
        println!("memory usage done .... 11111 ");
        thread::sleep(Duration::from_secs(10));
    }
}
