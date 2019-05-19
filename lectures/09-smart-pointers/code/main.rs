enum X<T> {
    X(T), Y
}

const C: X<T> = X::Y;

fn main() {}
