mod app;
mod callers;

fn main() {
    callers::starter::start_some_callers();
    callers::starter::start_some_callers_boxes();

    let call_once = {
        let x = 3;
        let s = String::from("hello");
        move || {
            println!("x is {}", x);
            println!("string is : {}", s);
            x
        }
    };
    call_once();
    call_once();

    let nums = vec![1, 2, 3, 4, 5];

    for num in nums {
        println!("num is {}", num);
    }

    // for 循环内部已经适应 into_iterator 转化，故循环后，nums 将不能使用
    // print!("nums is {}", nums[0]);

    // 使用into_iter 手工遍历
    let values = vec![1, 2, 3, 4, 5];
    let mut x = values.into_iter();

    loop {
        match x.next() {
            None => break,
            Some(v) => println!("value is {}", v),
        }
    }

    let values_2: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
    // map 可以用来使用迭代器结合闭包，修改数据
    let changed = values_2.into_iter().map(|x| x * 3);
    for v in changed {
        println!("v is {}", v);
    }

    // filter 用来筛选数据
    // map 用来修改数据
    // 最后，使用 collect 还原为 Vec 本身
    let values_3 = vec![1, 2, 3, 4, 5, 6];
    let filter_values: Vec<i32> = values_3.into_iter().filter(|x| *x > 3).collect();
    println!("filter result : ");
    for v in filter_values {
        println!("v is {}", v);
    }

    {
        let u = app::app::User::new(String::from("move"));
        // drop(u)
    }
}
