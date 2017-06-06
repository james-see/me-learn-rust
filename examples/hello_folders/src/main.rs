extern crate glob;
use self::glob::glob;

fn main() {
for entry in glob("/Users/jc/Downloads/**/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
}
