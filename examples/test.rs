extern crate tiny_ini;
use tiny_ini::Ini;

fn main() {
    let config = Ini::from_file("./example.ini");
    println!("config = {:?}", config);
    let s1 = config.section("section_one").unwrap();
    let n1: u32 = s1.get("name1").unwrap().parse().unwrap(); 
    println!("[section_one][name1] = {}", n1);
}