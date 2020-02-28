use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
    
    let stdout = stdout();
    let text = b"hello world";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(text, width, &mut writer).unwrap();
}
