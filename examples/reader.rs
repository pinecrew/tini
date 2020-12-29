extern crate tini;
use std::fs::File;
use std::io::BufReader;
use tini::Ini;

fn main() {
    let f = File::open("./examples/example.ini").unwrap();
    let mut reader = BufReader::new(f);
    let config = Ini::from_reader(&mut reader).unwrap();

    println!("{}", config.to_buffer());
}
