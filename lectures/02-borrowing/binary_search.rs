use std::cmp::Ordering;

fn binary_search(xs: &[i32], x: i32) -> bool {
    if xs.is_empty() {
        return false;
    }
    let mid = xs.len() / 2;
    let subslice = match xs[mid].cmp(&x) {
        Ordering::Less => &xs[mid + 1..],
        Ordering::Equal => return true,
        Ordering::Greater => &xs[..mid],
    };
    binary_search(subslice, x)
}

fn main() {}
