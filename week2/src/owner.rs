pub fn owner_test() {
    week1::mark_line("rust owner ");
    let a = "rust";
    println!("a is {}", a);
    println!("address of a is {:p}", a.as_ptr());
    let c = a;
    println!("address of c is {:p}", c.as_ptr());
    let mut b = a;
    println!("address of b is {:p}", b.as_ptr());
    b = "changed ...";
    println!("address of b is {:p}", b.as_ptr());

    let s = String::from("Stack string ??");
    println!("address of s is : {:p}", s.as_ptr());

    // let s2: String = s;

    use std::rc::Rc;
    let container;
    {
        let block_string: String = String::from("I'm string in block...");
        container = Rc::new(block_string);
        // Rc::new 也会发生所有权的转移，所以 block_string 将不在有效·
        // println!("block_string is {}", block_string);
    }

    let x = container.clone();
    println!("X is {}", x);

    // println!("address os s is : {:p}", s.as_ptr());
    // println!("address os s2 is : {:p}", s2.as_ptr());
    // println!("s2 is {:}", s2);
}
