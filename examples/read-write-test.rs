extern crate tini;
use tini::Ini;

static INPUT: &'static str = "./examples/example.ini";
static OUTPUT: &'static str = "./examples/test.ini";
static SPLIT: &'static str = "=----------------------------------------------------------=";

fn main() {
    let config = Ini::from_file(INPUT).unwrap();
    println!(">> read from `{0}`\n{1}\n{2}\n{1}", INPUT, SPLIT, config);

    let n1: u32 = config.get("section_one", "name1").unwrap_or(0);
    println!(">> entry `name1` from `section_one` = {}", n1);

    // read vector from ini
    let n2: Vec<bool> = config.get_vec("section_three", "frst5").unwrap_or(vec![false]);
    println!(">> entry `frst5` from `section_three` = {:?}", n2);

    // create Ini struture
    let test = Ini::new()
        // create section
        .section("section_one")
        // and add new item `a` and `b`
        .item("a", 1)
        .item("b", 2)
        // close previous section and create new one
        .section("section_two")
        .item("c", 3)
        .item("d", 4);
    println!(">> built `{0}` config\n{1}\n{2}\n{1}", OUTPUT, SPLIT, test);

    // write structure to `OUTPUT` file
    test.to_file(OUTPUT).unwrap();
}
