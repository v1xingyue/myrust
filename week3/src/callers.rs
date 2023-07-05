pub mod starter {
    use crate::app::app::{Hello, Hi, PluginCaller};
    pub fn start_some_callers() {
        let mut callers: Vec<&dyn PluginCaller> = vec![];
        let h = Hello::new("<<<<move>>>>>>".to_string());
        let h2 = Hi::new(12345);
        callers.push(&h);
        callers.push(&h2);

        for caller in callers {
            caller.do_action();
        }
    }

    pub fn start_some_callers_boxes() {
        let mut callers: Vec<Box<dyn PluginCaller>> = vec![];
        callers.push(Box::new(Hello::new(" move ".to_string())));
        callers.push(Box::new(Hi::new(12345677766)));
        for caller in callers {
            caller.do_action();
        }
    }
}
