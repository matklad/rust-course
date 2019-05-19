struct S<T> {
    t: T,
}

impl<T> Drop for S<T> {
    fn drop(&mut self) {}
}

fn main() {
    let (s, x): (S<_>, i32);
    x = 92;
    s = S { t: &x };
    drop(s)
}
