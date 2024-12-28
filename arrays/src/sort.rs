#[allow(dead_code)]
pub fn bubble_sort(list: &mut Vec<i32>) {
    for i in 0..list.len() {
        for j in 0..list.len() {
            if i != j && list[i] < list[j] {
                list.swap(i, j);
            }
        }
    }
}
