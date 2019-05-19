const HELLO: [u8; 5] = *b"hello";
const HELLO_2: [u8; 5] = [104, 101, 108, 108, 111];

fn main() {
    assert_eq!(HELLO, HELLO_2);
}
