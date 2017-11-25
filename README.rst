``me learn rust learn me``
=============
*Learning Rust my way.*

THINGS TO REMEMBER NOT TO FORGET:

1. Rust uses CARGO to manage builds and running Rust code. Do ``cargo init`` or ``cargo new --bin name_of_project`` for a new project that gets you started quickly and creates a Cargo.toml file. The nice thing is you can just run ``cargo run`` and it will auto-compile any changes.
2. CRATES (https://crates.io) are the packages for Rust. You can search them and use them by added them to the dependencies line in Cargo.toml.

EXAMPLES I WROTE:
~~~~~~~~~~~~~~~~~
1. hello_url - basic use case for urlparse and urljoin functionality in Rust (similar to python urllib.parse)
2. hello_html - basic use case to parse html (get text, get links, etc.)
3. hello_redis - basic use of redis as an example (connect)
4. hello_curl - basic use of tokio curl and curl in rust to collect a web page
5. guessing_game - keep guessing higher or lower to get number match correctly.
6. functions - example of how to use functions
7. loops - example of how to use loops with iter and without
8. variables - example of how to declare and use variables
9. FtoC - convert a random Fahrenheit temp to Celsius. 
