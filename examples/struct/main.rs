// TODO

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
        Ok(_config) => {
            // let vector2 = Vector2::deserialize(config).unwrap();

            // println!("{vector2:#?}");

            todo!()
        }
        Err(err) => println!("{err}"),
    }
}
