pub fn shift_string(s: &str, shift: usize) -> String {
    let len = s.len();
    if len == 0 {
        return s.to_string();
    }
    let shift = shift % len;
    let (left, right) = s.split_at(shift);
    format!("{}{}", right, left)
}

pub fn run() {
    let original = "abcdef";
    let shifted = shift_string(original, 2);
    println!("Original: {}", original);
    println!("Shifted: {}", shifted);
}