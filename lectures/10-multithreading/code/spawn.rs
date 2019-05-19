use std::thread;

fn main() {
    let handle: thread::JoinHandle<i32> = thread::spawn(|| { // <1>
        println!("Hello, world");
        92
    });
    let value = handle.join().unwrap(); // <2>
    assert_eq!(value, 92);
}
