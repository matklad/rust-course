use std::cell::Cell;

fn main() {
    let counter = Cell::new(0);
    crossbeam::scope(|s| {
        s.spawn(|_| {
            counter.set(counter.get() + 1)
        });
        counter.set(counter.get() + 1);
    }).unwrap();
    println!("{}", counter.get());
}
