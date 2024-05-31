use serde_lua::{value::IntoSerdeLua, LuaPestPair};

const SRC: &str = include_str!(".project.luau");

fn main() {
    let result = LuaPestPair::from_str(SRC);

    match result {
        Ok(config) => {
            println!("{:#?}", config.into_serde_lua());
        }
        Err(err) => println!("{err}"),
    }
}
