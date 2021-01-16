extern crate tini;
use std::collections::HashMap;
use tini::Ini;

fn main() {
    let config = Ini::from_file("converter.ini").unwrap();

    // iterate over config
    let currencies: HashMap<String, f64> =
        config.section_iter("currencies").map(|(k, v)| (k.to_string(), v.parse().unwrap())).collect();

    // cli code here
}
