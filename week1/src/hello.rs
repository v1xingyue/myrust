use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hello {
    msg: String,
    value: u32,
}

impl Hello {
    pub fn new(msg: String, value: u32) -> Self {
        return Self { msg, value };
    }
    pub fn say(self) {
        print!("this is method .. {},value is {} \n ", self.msg, self.value);
    }
    pub fn const_test() {
        const VER: u64 = 129;
        const MSG: &str = "hello world";
        println!("ver = {} , msg = {}!", VER, MSG);
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
