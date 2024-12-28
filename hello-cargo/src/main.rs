#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let square = Rectangle {
        height: 10,
        width: 10,
    };

    println!("{:?} have area equals to {}", &square, area(square));
}

fn area(rect: Rectangle) -> i32 {
    rect.height * rect.width
}

