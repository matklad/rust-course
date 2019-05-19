type Callback<'a> = Box<dyn Fn() + 'a>;

fn main() {
    let f: Callback<'static> =
        Box::new(|| println!("hello"));

    let x = 92;
    let g: Callback<'_> =
        Box::new(|| println!("x = {}", x));
}
