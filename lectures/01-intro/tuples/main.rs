fn main() {
    let t = (92,);
    println!("{:?}", &t as *const (i32,));
    println!("{:?}", &t.0 as *const i32);

    let x = 8;
    let y = (8, 264);
    let x_view: &[u8; 4] = unsafe { &*(&x as *const i32 as *const [u8; 4])};
    println!("{:02X?}", x_view);
    let y_view: &[u8; 8] = unsafe { &*(&y as *const (i32, i32) as *const [u8; 8])};
    println!("{:02X?}", y_view);
}
