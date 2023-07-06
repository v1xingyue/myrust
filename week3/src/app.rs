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

    impl From<String> for Hello {
        fn from(value: String) -> Self {
            Hello { name: value }
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

    // 从u64 转化为 Hi 类型
    impl From<u64> for Hi {
        fn from(value: u64) -> Self {
            return Hi { no: value };
        }
    }

    impl Into<u64> for Hi {
        fn into(self) -> u64 {
            self.no
        }
    }

    impl AsRef<Hi> for Hi {
        fn as_ref(&self) -> &Hi {
            self
        }
    }

    impl AsMut<Hi> for Hi {
        fn as_mut(&mut self) -> &mut Hi {
            self
        }
    }

    impl PluginCaller for Hi {
        fn do_action(&self) {
            println!("hello this contains no:  {}!", self.no);
        }
    }

    pub struct User {
        name: String,
    }

    impl Drop for User {
        fn drop(&mut self) {
            println!("I am dropped: {}", self.name);
        }
    }

    impl User {
        pub fn new(name: String) -> User {
            Self { name }
        }
    }
}
