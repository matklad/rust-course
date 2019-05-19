fn assign_ref<'a>(r: &mut &'a str, s: &'a str) {
    *r = s
}

fn evil(r: &mut &'static str) {
    let local: String = "spam".to_string();
    let local: &str = local.as_str();
    assign_ref(r, local)
}

fn main() {
    let mut hello: &'static str = "hello";
    evil(&mut hello);
    println!("{}", hello);
}
