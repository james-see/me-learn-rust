fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    let s3 = calc_reference_length(&s2);
    println!("The length of '{}' is {}.", s2, s3);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.
    (s, length)
    }

fn calc_reference_length(ss: &String) -> usize {
    ss.len()
}