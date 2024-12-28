fn main() {
    let n = 5;

    println!("number {} before increments: {}", n, increments(Some(n)));
}

fn increments(n: Option<i32>) -> i32 {
    match n {
        Some(n) => n + 1,
        None => 0,
    }
}