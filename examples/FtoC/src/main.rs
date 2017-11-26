use std::io;

fn main() {
    println!("Hello, converting F to C!");
    loop {
    println!("Fahrenheit value in decimal format: ");
    let mut guesser = String::new();
    io::stdin().read_line(&mut guesser).expect("Failed to read line");

    let guess: f64 = match guesser.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("this is the number: {}", guesser);
    
      //  let mut fahrenheit : f64 = rand::thread_rng().gen_range(1.0, 100.0);
    let mut fahrenheit : f64 = guess;
    //fahrenheit = fahrenheit;
    println!("Starting Fahrenheit temp to convert: {}", fahrenheit);
    let mut celsius : f64 = fahrenheit - 32.0;
    celsius = (celsius / 1.8 as f64);
    celsius = celsius.round();
    println!("Fahrenheit {} in Celsius: {}", fahrenheit,celsius);
    break;
    }
}
