extern crate rand;
use rand::Rng;

fn main() {
    println!("Hello, converting F to C!");
    let mut fahrenheit : f64 = rand::thread_rng().gen_range(1.0, 100.0);
    fahrenheit = fahrenheit.round();
    println!("Starting Fahrenheit temp to convert: {}", fahrenheit);
    let mut celcius : f64 = (fahrenheit - 32.0);
    celcius = (celcius / 1.8 as f64);
    celcius = celcius.round();
    println!("Fahrenheit {} in Celcius: {}", fahrenheit,celcius);
}
