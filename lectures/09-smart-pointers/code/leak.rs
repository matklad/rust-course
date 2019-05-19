impl<T> Box<T> {
    pub fn into_raw(b: Box<T>) -> *mut T { ... }
    pub unsafe fn from_raw(raw: *mut T) -> Self { ... }

    pub fn leak<'a>(b: Box<T>) -> &'a mut T
    {
        unsafe { &mut *Box::into_raw(b) }
    }
}
