use std::env;

#[test]
fn check_peasant_mult() {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let n1 = rng.gen_range(1, 10000);
        let n2 = rng.gen_range(1, 10000);
        assert_eq!(peasant_multiplication(n1, n2), n1 * n2);
    }
}

/// Adapted from reading "Algorithms" by Jeff Erickson
/// freely available on http://jeffe.cs.illinois.edu/teaching/algorithms/
pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} x y (two positive integers)", args[0]);
        return;
    }
    let x = args[1].parse::<u32>().unwrap();
    let y = args[2].parse::<u32>().unwrap();

    let answer = peasant_multiplication(x, y);
    println!("{} x {} = {}", x, y, answer);
}

/// Peasant Multiplication
/// Section 1.2, page 23
pub fn peasant_multiplication(x: u32, y: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    let x_prime = x / 2;
    let y_prime = y + y;
    let mut prod = peasant_multiplication(x_prime, y_prime);
    if x % 2 > 0 {
        prod += y;
    }
    return prod;
}
