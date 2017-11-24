use std::fs;
extern crate glob;
use self::glob::glob;

fn main() {

let files:Vec<Path> = glob("/Users/jc/Downloads/**/*.jpg").collect();

#for entry in glob("/Users/jc/Downloads/**/*.jpg").expect("Failed to read glob pattern") {
#    match entry {
#        Ok(path) => println!("{:?}", path.display()),
#        Err(e) => println!("{:?}", e),
#    }
#}
for entry in files {
	println!("{:?}",entry)
	}
}
