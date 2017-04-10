extern crate victoria_dom;

use victoria_dom::DOM;

fn main() {
    let html = r#"<html><div id="main">Hello, <a href="http://rust-lang.org" alt="The Rust Programing Language">Rust</a>. I am <a href="https://jamescampbell.us">here</a>.</div></html>"#;
    let dom = DOM::new(html);

    assert_eq!(dom.at("html").unwrap().text_all(), "Hello, Rust. I am here.");
    // get text from html
    let fulltext = dom.at("html").unwrap().text_all();
    println!("Text from html: {}", fulltext);
    assert_eq!(dom.at("div#main > a").unwrap().attr("alt").unwrap(), "The Rust Programing Language");
    // get first href
    let ahrefs = dom.at("div#main > a").unwrap().attr("href").unwrap().to_string();
    println!("first link: {}", ahrefs);
    // get all links into a vector
    let elems: Vec<_> = dom.find("div#main > a").iter().map(|x| x.attr("href").unwrap().to_string()).collect();
    println!("vector array of links printed: {:?}",elems);
    // loop through a vector and print
    for i in elems {
        println!("A link: {}",i);
    }
}

