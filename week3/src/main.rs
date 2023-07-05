mod app;
use app::app::{Hello, Hi, PluginCaller};

fn do_start_some_callers() {
    let h = Hello::new("xingyue".to_string());
    h.start();
    h.do_action();

    let mut callers: Vec<&dyn PluginCaller> = vec![];
    let h = Hello::new("move".to_string());
    let h2 = Hi::new(12345);

    callers.push(&h);
    callers.push(&h2);

    let mut boxes_callers: Vec<Box<dyn PluginCaller>> = vec![];
    boxes_callers.push(Box::new(Hello::new("move".to_string())));
    boxes_callers.push(Box::new(Hello::new("funs".to_string())));
    boxes_callers.push(Box::new(Hi::new(123333333)));

    for caller in boxes_callers {
        caller.do_action();
    }

    for caller in callers {
        caller.do_action();
    }
}

fn main() {
    do_start_some_callers();
    let c1 = |msg: &str| println!("this is fn world {}", msg);

    let c2 = |msg: &mut u64| println!("this is fn world {}", msg);
    {
        let msg = "hello world";
        c1(msg);
    }
    {
        let msg: &mut u64 = &mut 123;
        c2(msg);
    }

    println!("hello world");
}
