fn main() {
    mod m {
        pub(super) fn hello() {
            println!("Hello, world!");
        }
    }
    m::hello();
}
