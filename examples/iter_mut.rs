extern crate tini;
use tini::Ini;

fn main() {
    let mut config = Ini::new().section("items").item("a", "1").item("b", "2").item("c", "3");

    // mutate items
    for (_, item) in config.iter_mut() {
        for (_, value) in item {
            let v: i32 = value.parse().unwrap();
            *value = format!("{}", v + 1);
        }
    }

    println!("{}", config.to_buffer());
}
