pub mod main {

    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    pub fn start() {
        print!("hello public world...");

        print!("game started , input a number : \n ");

        let secet_number = rand::thread_rng().gen_range(1, 100);
        println!("secet_number is {}", secet_number);

        loop {
            let mut line = String::new();
            let input_size: usize = io::stdin()
                .read_line(&mut line)
                .expect("You must input a number.");

            print!("you input size : {}", input_size);

            let input_num: u64 = match line.trim().parse() {
                Ok(v) => v,
                Err(err) => {
                    println!("{}:", err);
                    continue;
                }
            };

            println!("line is -> {}", line);

            match input_num.cmp(&secet_number) {
                Ordering::Less => println!("too less"),
                Ordering::Equal => {
                    println!("you win!!!");
                    break;
                }
                Ordering::Greater => println!("too bigger"),
            }
        }
    }
}
