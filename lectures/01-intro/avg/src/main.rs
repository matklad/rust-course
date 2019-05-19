fn main() {
    const N: usize = 100000000;
    let mut xs = vec![0; N];
    let mut random = N as i32;
    for i in 0..N {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        xs[i] = random;
    }
    for i in 0..100 {
        let start = std::time::Instant::now();
        let res = average2(&xs);
        let elapsed = start.elapsed();
        if i % 10 == 0 {
            println!("{} {}", res, elapsed.subsec_millis());
        }
    }
}

fn average(xs: &[i32]) -> f64 {
    let mut sum: i32 = 0;
    for i in 0..xs.len() {
        sum += xs[i];
    }
    sum as f64 / xs.len() as f64
}

fn average2(xs: &[i32]) -> f64 {
    xs.iter().fold(0, |x, y| x + y) as f64 / xs.len() as f64
}
