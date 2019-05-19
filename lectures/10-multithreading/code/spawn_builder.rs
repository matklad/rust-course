use std::thread;

fn main() {
    let handle = thread::Builder::new()
        .name("my-thread".into())
        .stack_size(32 * 1024)
        .spawn(|| {
            println!("Hello, world");
            92
        }).unwrap();
    let value = handle.join().unwrap();
    assert_eq!(value, 92);
}
