extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    let x = unsafe { add(62, 30) };
    println!("{}", x);
}