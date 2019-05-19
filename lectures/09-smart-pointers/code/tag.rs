trait Tagged {
    const TAG: &'static str;
}

struct Foo;
impl Tagged for Foo { const TAG: &'static str = "Foo"; }

struct Bar;
impl Tagged for Bar { const TAG: &'static str = "Bar"; }


fn by_tag(tag: &str) {
    match tag {
        Foo::TAG => println!("foo"),
        Bar::TAG => println!("bar"),
        _ => panic!("unknown tag: {:?}", tag)
    }
}

