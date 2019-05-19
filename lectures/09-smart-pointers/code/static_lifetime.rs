const X: i32 = 92;
const R: &'static i32 = &X;

fn foo(x: &i32) {
    println!("{}", x);
}

fn main() {
    foo(R);
}
