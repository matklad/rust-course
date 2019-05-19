use std::rc::Rc;

fn main() {
    let hello = Rc::new(String::from("hello"));
    let thread = std::thread::spawn({
        let hello = Rc::clone(&hello);
        move || {
            println!("{}", hello)
        }
    });
    thread.join().unwrap();
}
