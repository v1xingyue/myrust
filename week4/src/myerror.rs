pub mod myerror {

    fn return_option(num: u64) -> Option<u64> {
        if num >= 2000 {
            Some(num)
        } else {
            None
        }
    }

    fn return_error(a: u64, b: u64) -> Result<u64, String> {
        if b != 0 {
            Ok(a / b)
        } else {
            Err(String::from("b can't be 0 "))
        }
    }

    pub fn option_test() {
        let s: Option<u64> = return_option(1000);
        match s {
            None => println!("return none"),
            Some(x) => println!("value is {}", x),
        }
    }

    pub fn error_test() {
        match return_error(3, 1) {
            Err(msg) => println!("msg is {}", msg),
            Ok(val) => println!("value is {}", val),
        }

        match return_error(3, 0) {
            Err(msg) => println!("error : msg is {}", msg),
            Ok(val) => println!("value is {}", val),
        }
    }

    pub fn assert_test(v: u64, z: u64) {
        assert!(v != 0);
        println!("v is not 0 , can run here.");
        assert_eq!(v, v);
        assert_ne!(v, v + z);

        if 10 < (v + z) {
            panic!("should Break with panic ");
        }
    }

    pub fn my_add(a: u64, b: u64) -> u64 {
        a + b
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_hello() {
        assert_eq!(1, 1);
    }
}
