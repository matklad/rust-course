fn main() {
    let x = i32::max_value();
    // let y = x + 1;
    // println!("{}", y);
    let x = i32::max_value();
    let y = x.wrapping_add(1);
    assert_eq!(y, i32::min_value());
    let y = x.saturating_add(1);
    assert_eq!(y, i32::max_value());
    let (y, overflowed) = x.overflowing_add(1);
    assert!(overflowed);
    assert_eq!(y, i32::min_value())
}
