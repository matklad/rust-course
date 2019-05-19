fn foo() {
    let x = 92;
    std::thread::spawn(move || {
        println!("{}", x);
    });
}
