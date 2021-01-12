extern crate tini;
use tini::Ini;

fn main() {
    let v_string = vec!["1".to_owned(), "2".to_owned(), "3".to_owned()];
    let v_str_1 = vec!["a", "b", "c"];
    let v_str_2 = ["r", "u", "s", "t"];
    let v_int = [1, 2, 3];
    let v_q = vec!["a,b", "c,d", "e,f"];

    let conf = Ini::new()
        .section("demo")
        // add String vector
        .item_vec("vec_string", &v_string)
        // add &str vector
        .item_vec("vec_str", &v_str_1)
        // add &str slice
        .item_vec("str", &v_str_2)
        // and int slice
        .item_vec("int", &v_int)
        // with another separator symbol
        .item_vec_with_sep("quoted", &v_q, "| ")
        // and combined
        .item_vec_with_sep("a", &["a,b", r"\", "c,d", "e"], "| ");

    println!("{}", conf);
}
