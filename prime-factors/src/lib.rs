pub fn factors(n: u64) -> Vec<u64> {
    let mut remainder = n;
    let mut prime_factors = Vec::new();
    let mut i = 0;
    loop {
        if remainder == 1 {
            return prime_factors;
        }
        let prime = nth(i);
        if remainder % prime == 0 {
            prime_factors.push(prime);
            remainder /= prime;
        } else {
            i += 1;
        }
    }
}

pub fn nth(n: u64) -> u64 {
    let mut i = 0;
    let mut test_number = 2;
    loop {
        if is_prime(test_number) {
            if i == n {
                return test_number;
            }
            i += 1;
        }
        test_number += 1;
    }
}

fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n == 4 {
        return false;
    }
    for i in 2..n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
