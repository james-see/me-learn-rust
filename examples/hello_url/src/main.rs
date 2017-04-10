extern crate url;
use url::Url;

fn main() {
let this_document = Url::parse("http://servo.github.io/rust-url/url/index.html").unwrap();
let css_url = this_document.join("../main.css").unwrap();
assert_eq!(css_url.as_str(), "http://servo.github.io/rust-url/main.css");
println!("fixed auto: {}", css_url.as_str());
}

