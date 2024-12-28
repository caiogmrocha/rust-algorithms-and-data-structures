pub fn fatorial(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

pub fn is_prime(n: i32) -> bool {
    for i in 2..n-1 {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}