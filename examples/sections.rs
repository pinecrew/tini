extern crate tini;
use std::collections::HashMap;
use tini::Ini;

fn main() {
    let mut config = Ini::new()
        // crate section
        .section("items")
        // add items
        .item("a", 1)
        .item("b", 2)
        .item("c", 3)
        // create another
        .section("values")
        // and add multiple values at a time
        .items(vec![
            ("d", 4),
            ("e", 5),
            ("f", 6),
        ]);

    println!("before delete:\n-----\n{}\n-----\n", config);

    config = config
        // select section
        .section("values")
        // and remove it from config
        .clear()
        // select another section
        .section("items")
        // and remove item from it
        .erase("c");

    println!(" after delete:\n-----\n{}\n-----\n", config);

    // create custom section using HashMap
    let mut section = HashMap::new();
    section.insert("val", 42);
    // and add
    config = config.section("hashmap").items(section);

    println!(" after insert:\n-----\n{}\n-----", config);
}
