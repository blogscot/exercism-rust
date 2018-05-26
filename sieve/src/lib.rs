pub fn primes_up_to(limit: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = (1..=limit).collect();
    primes[0] = 0;

    for value in series(limit) {
        for i in range_step(value + 1, limit, value) {
            primes[i - 1] = 0;
        }
    }
    primes.into_iter().filter(|&x| x > 0).collect()
}

fn series(limit: usize) -> Vec<usize> {
    (2..).take_while(|x| x * x <= limit).collect()
}

fn range_step(start: usize, end: usize, step: usize) -> Vec<usize> {
    (start..=end).filter(|x| x % step == 0).collect()
}
