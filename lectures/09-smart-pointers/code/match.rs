fn foo(x: i32) {
    const ZERO: i32 = 0;

    match x {
        ZERO => println!("zero"), // <1>
        other => println!("other: {}", other), // <2>
    }
}
