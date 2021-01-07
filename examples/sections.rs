extern crate tini;
use std::collections::HashMap;
use tini::Ini;

fn main() {
    let mut config = Ini::new()
        .section("items")
        .item("a", "1")
        .item("b", "2")
        .item("c", "3")
        .section("values")
        .items(vec![("d", "4"),
                    ("e", "5"),
                    ("f", "6")]);

    println!("before delete:\n-----\n{}\n-----\n", config);

    config = config.section("values").clear()
                   .section("items").remove("c");

    println!(" after delete:\n-----\n{}\n-----\n", config);

    // create custom section using HashMap
    let mut section = HashMap::new();
    section.insert("val", 42);
    // and add
    config = config.section("hashmap").items(section);

    println!(" after insert:\n-----\n{}\n-----", config);
}
