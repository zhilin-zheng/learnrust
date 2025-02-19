use std::env;
use std::str::FromStr;

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
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d)
    // let x: u64 = 4;
    // let y: u64 = 6;
    // let divisor = gcd(x, y);
    /*println!(
        "The greatest common divisor of {} and {} is: {}\n",
        x, y, divisor
    );*/
}
#[test]
fn test_gcd() {
    assert_eq!(gcd(333, 444), 111);
    assert_eq!(gcd(3 * 11 * 17 * 19, 2 * 11 * 17 * 29), 11 * 17);
    assert_eq!(gcd(1, 11), 1);
}
