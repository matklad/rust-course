pub fn library_fn<T, F: FnOnce() -> T>(user_code: F) -> T {
    let _guard = Cleanup; // <1>
    user_code()
}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("cleanup!")
    }
}
