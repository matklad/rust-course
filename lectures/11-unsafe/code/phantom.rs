use std::marker::PhantomData;

struct IterMut<'a, T> {
    begin: *mut T,
    end: *mut T,
    slice: PhantomData<&'a mut [T]>,
}

impl<'a, T> IterMut<'a, T> {
    fn new(slice: &'a mut [T]) -> Self {
        assert!(std::mem::size_of::<T>() > 0);
        let begin = slice as *mut [T] as *mut T;
        let end = unsafe { begin.add(slice.len()) };
        IterMut {
            begin,
            end,
            slice: PhantomData,
        }
    }
}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.begin == self.end {
            return None;
        }
        let curr = self.begin;
        unsafe {
            self.begin = self.begin.add(1);
            Some(&mut *curr)
        }
    }
}
