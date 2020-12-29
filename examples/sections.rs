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
        .item("d", "4")
        .item("e", "5")
        .item("f", "6");

    println!("before delete:\n-----\n{}\n-----\n", config);

    let mut section = config.remove_section("values").unwrap();
    config.remove_item("items", "c");

    println!(" after delete:\n-----\n{}\n-----\n", config);

    // update removed section
    section.insert(String::from("g"), String::from("42"));
    // and add with different name
    config.insert_section("vals", section);

    // create custom section using HashMap
    let mut section = HashMap::new();
    section.insert("val", 42);
    // and add
    config.insert_section("hashmap", section);

    println!(" after insert:\n-----\n{}\n-----", config);
}
