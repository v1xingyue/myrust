pub mod app {
    pub struct Hello {
        name: String,
    }

    pub trait PluginCaller {
        fn do_action(&self);
    }

    impl Hello {
        pub fn new(name: String) -> Self {
            Hello { name }
        }
        pub fn start(&self) {
            println!("hello start {}!", self.name);
        }
    }

    impl PluginCaller for Hello {
        fn do_action(&self) {
            println!("hello this plugin method {}!", self.name);
            self.start();
        }
    }

    pub struct Hi {
        no: u64,
    }

    impl Hi {
        pub fn new(no: u64) -> Self {
            Hi { no }
        }
    }

    impl PluginCaller for Hi {
        fn do_action(&self) {
            println!("hello this contains no:  {}!", self.no);
        }
    }
}
