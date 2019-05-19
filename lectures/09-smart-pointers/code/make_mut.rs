use std::rc::Rc;

#[derive(Clone)]
enum Stack<T> {
    Empty,
    Cons(T, Rc<Stack<T>>)
}

impl<T> Stack<T> {
    fn push(&mut self, value: T) {
        let this = std::mem::replace(self, Stack::Empty);
        *self = Stack::Cons(value, Rc::new(this));
    }

    fn pop(&mut self) -> Option<T> {
        match self {
            Stack::Empty => None,
            Stack::Cons(value, tail) => {
                *self = tail;
                Some(value)
            }
        }
    }
}

fn main() {}
