pub struct Person {
    name: String,
    age: u8,
}

pub impl Person {
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}