#[allow(dead_code)]
pub fn linear_search<T: std::cmp::PartialOrd>(list: Vec<T>, value: T) -> Option<T> {
    for i in list {
        if i == value {
            return Some(i);
        }
    }

    None
}

#[allow(dead_code)]
pub fn binary_search<T: std::cmp::PartialOrd>(list: &[T], value: T) -> Option<&T> {
    let middle_index = list.len() / 2;

    if list.len() == 1 {
        if list[0] == value {
            return Some(&list[0]);
        } else {
            return None;
        }
    }

    if list[middle_index] == value {
        return Some(&list[middle_index]);
    } else if list[middle_index] < value {
        return binary_search(&list[middle_index..list.len()], value);
    } else {
        return binary_search(&list[0..middle_index], value);
    }
}

