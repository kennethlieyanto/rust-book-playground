fn main() {
    println!("{}", fibonacci(7));
}

fn fibonacci(nth: u64) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for _ in 0..nth {
        let temp = a + b;
        a = b;
        b = temp;
    }

    a
}
