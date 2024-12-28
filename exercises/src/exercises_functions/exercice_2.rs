pub fn revert(original: String) -> String {
    let mut output = String::new();
    let chars: Vec<char> = original.chars().collect();

    for i in (0..chars.len()).rev() {
        output.push(chars[i]);
    }

    output
}