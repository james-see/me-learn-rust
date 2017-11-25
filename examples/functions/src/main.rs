fn main() {
    let lucky_number = 7; // I'm feeling lucky today.
    println!("Hello, world!");
    another_function(5, 6);
    let x = five();

    if x < 5 {
         println!("condition was true");
     } else {
         println!("condition was false");
    }

    println!("The value of x is: {}", x);
    looper_down(10);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn looper_down(x: i32) {
    let mut number = x;
    while number != 0  {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}
