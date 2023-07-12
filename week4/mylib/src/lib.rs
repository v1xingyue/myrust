use http_req::request;
use http_req::response::Response;

pub fn add(left: usize, right: usize) -> usize {
    println!("now calculate {} + {} = {}", left, right, left + right);

    left + right
}

pub fn sui_get() -> (Vec<u8>, Response) {
    let mut buffer = Vec::new(); //container for body of a response
    let response = request::get("https://www.baidu.com/", &mut buffer).unwrap();
    (buffer, response)
}

pub fn get_page() -> (Vec<u8>, Response) {
    let mut buffer = Vec::new(); //container for body of a response
    let response = request::get("https://www.baidu.com/", &mut buffer).unwrap();
    (buffer, response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_http_request() {
        let (body, response) = get_page();
        println!("status code: {}", response.status_code());
        println!("body is {}", String::from_utf8(body).unwrap());
    }
}
