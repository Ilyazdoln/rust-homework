pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}

pub fn run() {
    let num = 12321;
    println!("Is {} a palindrome? {}", num, is_palindrome(num));
}