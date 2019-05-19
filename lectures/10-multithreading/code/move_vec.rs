fn main() {
    let mut xs = vec![1, 2, 3];
    std::thread::spawn(move || {
        xs.push(4);
        println!("{:?}", xs);
    });
}
