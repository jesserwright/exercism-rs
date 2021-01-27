pub fn nth(n: u32) -> u32 {
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

fn is_prime(n: u32) -> bool {
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

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(13), true);
    assert_eq!(is_prime(104_743), true);
}
