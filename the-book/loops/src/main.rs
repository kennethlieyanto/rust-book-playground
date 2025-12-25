fn main() {
    let temp_in_f = 100.0;

    println!("100F in celcius is {}", to_celcius(temp_in_f))
}

fn to_celcius(temp_in_f: f64) -> f64 {
    (temp_in_f - 32.0) * 5.0 / 9.0
}
