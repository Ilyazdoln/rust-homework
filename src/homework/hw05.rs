pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn run() {
    let a = 48;
    let b = 18;
    let result = gcd(a, b);
    println!("GCD of {} and {} is {}", a, b, result);
}