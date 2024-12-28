mod math;

fn main() {
    let v1 = math::Vector::new(1.0, 1.0);
    let v2 = math::Vector::new(2.0, 2.0);

    println!("v1 == v2 -> {}", v1 == v2);
}
