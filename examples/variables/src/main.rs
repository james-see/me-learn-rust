fn main() {
    const MAX_POINTS: u32 = 100_000;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {} and max points: {}", x,MAX_POINTS);

    let heart_eyed_cat = '😻';
    println!("{}",heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    let five_hundred = tup.0;

    println!("The value of the first in tup is: {}", five_hundred);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("First in array is: {}",first);

    another_function();

}

fn another_function() {
    println!("Another function.");
}