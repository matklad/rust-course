fn fib(n: usize) -> usize {
    let mut cache = vec![0; n + 1];
    return fib_memo(&mut cache, n);

    fn fib_memo(cache: &mut Vec<usize>, n: usize) -> usize {
        if n == 0 || n == 1 {
            return 1;
        }
        if cache[n] == 0 {
            cache[n] =
                fib_memo(cache, n - 1) + fib_memo(cache, n - 2);
        }
        cache[n]
    }
}
