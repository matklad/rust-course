use std::cell::{RefCell, Ref};

#[derive(Default)]
struct Person { first_name: String, last_name: String }

fn main() {
    let cell = RefCell::new(Person::default());
    let p: Ref<Person> = cell.borrow();
    let first_name: Ref<str> =
        Ref::map(p, |it| it.first_name.as_str());
}
