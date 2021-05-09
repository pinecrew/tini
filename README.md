# tini &mdash; A tiny INI parsing library

![Rust](https://github.com/pinecrew/tini/workflows/Rust/badge.svg)
[![Crates](https://img.shields.io/crates/v/tini.svg)](https://crates.io/crates/tini)
[![Docs](https://docs.rs/tini/badge.svg)](https://docs.rs/tini)

## Usage

Add `tini` to your `Cargo.toml`, for example:
```toml
[dependencies]
tini = "1.1"
```

## How to use

#### Read ini configuration from file

```rust
extern crate tini;
use tini::Ini;

fn main() {
    // Read example.ini file from examples directory
    let config = Ini::from_file("./examples/example.ini").unwrap();
    // Read name3 key from section_one
    let name3: String = config.get("section_one", "name3").unwrap();
    // Read list of values
    let frst5: Vec<bool> = config.get_vec("section_three", "frst5").unwrap();
    println!("name3 = {}", name3);
    println!("frst5 = {:?}", frst5);
    // Result:
    // name3 = example text
    // frst5 = [true, false, true]
}
```

#### Create ini configuration and write to file

```rust
extern crate tini;
use tini::Ini;

fn main() {
    // Create ini structure
    let conf = Ini::new()                                          // initialize Ini
                   .section("params")                              // create `params` section
                   .item("pi", 3.14)                               // add `pi` key
                   .item_vec("lost", &[4, 8, 15, 16, 23, 42])      // add `lost` list
                   .section("other")                               // create another section
                   .item("default", "hello world!");               // add `default` key to `other` section
    // At any time you can add new parameters to the last created section
    // < some code >
    // Now write ini structure to file
    conf.to_file("output.ini").unwrap();
    // Now `output.ini` contains
    // -----------------------------
    // [params]
    // pi = 3.14
    // lost = 4, 8, 15, 16, 23, 42
    //
    // [other]
    // default = hello world!
    // -----------------------------
}
```

See more examples in [documentation](https://docs.rs/tini).
