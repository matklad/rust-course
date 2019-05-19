#[derive(Debug)]
struct Color {
    r: u8, g: u8, b: u8
}

const BLACK: Color = Color { r: !0, g: !0, b: !0 };

fn main() {
    println!("men in {:?}", BLACK)
}
