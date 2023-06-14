pub mod types {

    pub struct Wrapper<T> {
        data: T,
    }

    impl<T> Wrapper<T> {
        pub fn hello(&self) -> &T {
            &self.data
        }
        pub fn hello_mut(&mut self) -> &mut T {
            &mut self.data
        }

        pub fn get_data(&self) -> &T {
            &self.data
        }
    }

    pub fn new<T>(data: T) -> Wrapper<T> {
        return Wrapper { data };
    }
}
