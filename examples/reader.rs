extern crate tini;
use std::fs::File;
use std::io::BufReader;
use tini::Ini;

fn main() {
    // Opening the file in any convenient way
    let f = File::open("./examples/example.ini").unwrap();
    // Create reader from file
    let mut reader = BufReader::new(f);
    // Create config from BufReadear 
    let config = Ini::from_reader(&mut reader).unwrap();
    // Serialize to stdout
    println!("{}", config);
}
