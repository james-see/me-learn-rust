extern crate rand;
use rand::Rng;

fn main() {
    println!("Hello, converting F to C!");
    let mut fahrenheit : f64 = rand::thread_rng().gen_range(1.0, 100.0);
    fahrenheit = fahrenheit.round();
    println!("Starting Fahrenheit temp to convert: {}", fahrenheit);
    let mut celsius : f64 = (fahrenheit - 32.0);
    celsius = (celsius / 1.8 as f64);
    celsius = celsius.round();
    println!("Fahrenheit {} in Celsius: {}", fahrenheit,celsius);
}
