pub fn count_words(string: String) -> i32 {
    let mut count = 0;

    if !string.is_empty() {
        count += 1;
    }

    for c in string.chars() {
        if c == ' ' {
            count += 1;
        }
    }

    count
}