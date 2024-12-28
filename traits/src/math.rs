use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Vector {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    pub fn size(&self) -> f64 {
        f64::sqrt(f64::powf(self.x, 2.0) + f64::powf(self.y, 2.0))
    }
}

impl std::cmp::PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size() && self.x == self.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl std::cmp::PartialOrd for Vector {
    fn gt(&self, other: &Self) -> bool {
        self.size() > other.size()
    }

    fn ge(&self, other: &Self) -> bool {
        self.size() >= other.size()
    }

    fn lt(&self, other: &Self) -> bool {
        self.size() < other.size()
    }

    fn le(&self, other: &Self) -> bool {
        self.size() <= other.size()
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.gt(other) {
            return Some(Ordering::Greater);
        } else if self.lt(other) {
            return Some(Ordering::Less);
        } else if self.eq(other) {
            return Some(Ordering::Equal);
        } else {
            None
        }
    }
}
