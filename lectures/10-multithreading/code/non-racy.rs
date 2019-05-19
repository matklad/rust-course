use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let mut counter = AtomicUsize::new(0);
    crossbeam::scope(|s| {
        s.spawn(|_| {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        counter.fetch_add(1, Ordering::SeqCst);
    }).unwrap();
    assert_eq!(*counter.get_mut(), 2)
}
