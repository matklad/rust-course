const FOO: i32 = 92;  // value
type FOO = ();        // type

struct Bar { f: u32 }    // type
fn Bar(f: u32) -> Bar  { // value
    Bar { f }
}

mod m {}  // type
fn m() {} // value
