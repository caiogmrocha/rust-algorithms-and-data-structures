pub fn sum(nums: &[i32]) -> i32 {
    let mut s = 0;

    for n in nums {
        s += n;
    }

    s
}