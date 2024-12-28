pub fn is_prime(num: i32) -> bool {
    let mut is = true;

    for i in 2..num {
        if num % i == 0 {
            is = false;
        }
    }

    is
}