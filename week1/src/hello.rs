use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hello {
    msg: String,
    value: u32,
    version: u32,
    params: Vec<String>,
}

impl Hello {
    pub fn new(msg: String, value: u32) -> Self {
        return Self {
            msg,
            value,
            version: 0,
            params: vec!["hello".to_string(), "world".to_string()],
        };
    }
    pub fn say(&self) {
        println!("this is method .. {},value is {} \n ", self.msg, self.value);
        let msg = serde_json::to_string(self).unwrap();
        println!("json message : {}", msg)
    }
    pub fn const_test() {
        const VER: u64 = 129;
        const MSG: &str = "hello world";
        println!("ver = {} , msg = {}!", VER, MSG);
    }

    pub fn print_json(&self) {
        print!("this is json content .. \n");
    }

    pub fn other_test() {
        let hello = "world";
        println!("{}", hello);

        let mut v: u32 = 128;
        println!("now v = {}", v);
        v = 256;
        println!("now v = {}", v);

        let mut p: &u32 = &128;

        println!("p = {}", p);

        println!("now I will change value of p to");
        p = &256;
        println!("p = {}", p);

        let mut test_string = "hello world".to_string();
        println!("testString is from  {}", test_string);
        test_string = "does this can change ???".to_string();

        println!("testString is to:  {}", test_string);
    }
}
