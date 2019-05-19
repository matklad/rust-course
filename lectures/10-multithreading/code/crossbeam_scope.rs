fn main() {
    let x = 92;
    crossbeam::scope(|s| {
        s.spawn(|_| { // <1>
            println!("{}", x)
        });
    }).unwrap();
}
