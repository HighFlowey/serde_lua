use std::fs;

use serde_lua::LuaPestPair;

const SRC: &str = include_str!("input.project.luau");

fn main() {
    let result = LuaPestPair::from_str(SRC);

    match result {
        Ok(config) => {
            fs::write(
                "examples/rojo/out.project.json",
                serde_json::to_string_pretty(&config).expect("Failed to deserialize"),
            )
            .expect("Failed to write file");
        }
        Err(err) => println!("{err}"),
    }
}
