use ferris_says::say;
use std::io::{stdout, BufWriter};


fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans! Hello Hello Hello Hello ");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}