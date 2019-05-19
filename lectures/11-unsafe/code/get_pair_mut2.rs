fn get_pair_mut<T>(
    xs: &mut [T],
    idx1: usize,
    idx2: usize,
) -> (&mut T, &mut T) {
    let (idx1, idx2) = (idx1.min(idx2), idx1.max(idx2));
    let (lo, hi) = xs.split_at_mut(idx2);
    (&mut lo[idx1], &mut hi[0])
}
