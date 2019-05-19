fn foo() {
    let x = 92;
// closure may outlive the current function, but it borrows `x`, which is owned by the current function
    std::thread::spawn(|| {
        println!("{}", x);
    });
}

fn main() {}
