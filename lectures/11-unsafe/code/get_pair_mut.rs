fn get_pair_mut<T>(
    xs: &mut [T],
    idx1: usize,
    idx2: usize,
) -> (&mut T, &mut T) {

    let x1: *mut T = &mut xs[idx1];
    let x2: *mut T = &mut xs[idx2];

    unsafe {
        assert!(idx1 != idx2);
        (&mut *x1, &mut *x2)
    }
}
