extern crate tini;
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

    section.insert(String::from("g"), String::from("42"));
    config.insert_section("vals", &section);

    println!(" after insert:\n-----\n{}\n-----", config);
}
