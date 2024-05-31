use serde_lua::LuaPestPair;

const SRC: &str = include_str!(".project.luau");

fn main() {
    let result = LuaPestPair::from_str(SRC);

    match result {
        Ok(config) => {
            println!("{}", serde_json::to_string_pretty(&config).unwrap());
        }
        Err(err) => println!("{err}"),
    }
}
