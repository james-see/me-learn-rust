fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    } 
    liftoff(7);
}

fn liftoff(max: i32) {
    for number in (1..max).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}