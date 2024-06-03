// TODO

use serde::Deserialize;
use serde_derive::Deserialize;
use serde_lua::LuaPestPair;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Vector2 {
    x: f64,
    y: f64,
}

const SRC: &str = include_str!("struct.luau");

fn main() {
    let result = LuaPestPair::from_str(SRC);

    match result {
        Ok(config) => {
            let json = config.into_serde_json().unwrap();
            let vector2 = Vector2::deserialize(json).unwrap();
            println!("{vector2:#?}");
        }
        Err(err) => println!("{err}"),
    }
}
