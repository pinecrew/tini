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

    println!("before delete:\n-----\n{}\n-----", config.to_buffer());

    config.delete_section("values");
    config.delete_item("items", "c");

    println!(" after delete:\n-----\n{}\n-----", config.to_buffer());
}
