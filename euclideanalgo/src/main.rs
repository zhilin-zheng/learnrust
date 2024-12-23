fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
fn main() {
    let x: u64 = 4;
    let y: u64 = 6;
    let divisor = gcd(x, y);
    println!(
        "The greatest common divisor of {} and {} is: {}\n",
        x, y, divisor
    );
}