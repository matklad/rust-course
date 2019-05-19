use std::rc::Rc;

fn main() {
    let hello1: Rc<String> = Rc::new("hello".to_string());
    let hello2 = Rc::clone(&hello);
}
